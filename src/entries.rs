use crate::{Result, error};

fn data_path() -> Result<&'static std::path::PathBuf> {
    static PATH: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        dirs::data_local_dir()
            .filter(|p| p.is_dir())
            .map(|p| p.join(env!("CARGO_PKG_NAME")).join("entries"))
    })
    .as_ref()
    .ok_or(error::Error::DataPath)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, bincode::Encode, bincode::Decode)]
pub struct Entry {
    path: std::path::PathBuf,
    last_visit: u64,
}

pub struct Access(std::path::PathBuf);

impl TryFrom<String> for Access {
    type Error = error::Error;

    fn try_from(value: String) -> Result<Self> {
        let path = std::path::PathBuf::from(value);

        let path = path
            .canonicalize()
            .map_err(|_| error::Args::InvalidPath(path))?;

        if !path.is_dir() {
            return Err(error::Error::Args(error::Args::InvalidPath(path)));
        }

        let path = if let Some(home) = dirs::home_dir() {
            if let Ok(stripped) = path.strip_prefix(home) {
                std::path::Path::new("~").join(stripped)
            } else {
                path
            }
        } else {
            path
        };

        Ok(Self(path))
    }
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct Entries(Vec<Entry>);

impl Entries {
    const CONFIG: bincode::config::Configuration<
        bincode::config::LittleEndian,
        bincode::config::Fixint,
    > = bincode::config::standard().with_fixed_int_encoding();

    pub fn new() -> Result<Self> {
        let path = data_path()?;
        let file = std::fs::OpenOptions::new().read(true).open(path)?;

        fs2::FileExt::lock_shared(&file)?;

        let mut reader = std::io::BufReader::new(file);
        bincode::decode_from_std_read(&mut reader, Self::CONFIG).map_err(error::Error::Decode)
    }

    pub fn write(access: Access) -> Result {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())?;

        let entry = Entry {
            path: access.0,
            last_visit: now,
        };

        let path = data_path()?;
        if !path.exists() {
            std::fs::create_dir(
                dirs::data_local_dir()
                    .map(|p| p.join(env!("CARGO_PKG_NAME")))
                    .ok_or(error::Error::DataPath)?,
            )
            .map_err(error::Error::DataPathInit)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        fs2::FileExt::lock_exclusive(&file)?;

        let mut entries = {
            let mut reader = std::io::BufReader::new(&file);
            bincode::decode_from_std_read::<Vec<Entry>, _, _>(&mut reader, Self::CONFIG)
                .ok()
                .unwrap_or_else(Vec::new)
        };
        println!("entries.len() = {}", entries.len());
        println!("Loaded   {entries:?}");

        entries.retain(|e| now - e.last_visit < 60 * 60 * 24 * 365);
        println!("Retained {entries:?}");

        #[cfg(feature = "check_on_load")]
        {
            entries.sort_unstable_by(|a, b| match a.path.cmp(&b.path) {
                std::cmp::Ordering::Equal => b.last_visit.cmp(&a.last_visit),
                o => o,
            });
            println!("Sorted   {entries:?}");
            entries.dedup_by(|a, b| a.path == b.path);
            println!("Deduped  {entries:?}");
        }

        match entries.binary_search_by(|e| e.path.cmp(&entry.path)) {
            Ok(idx) => unsafe { *entries.get_unchecked_mut(idx) = entry },
            Err(idx) => entries.insert(idx, entry),
        }
        println!("Pushed   {entries:?}");

        let mut writer = std::io::BufWriter::new(file);
        bincode::encode_into_std_write(entries, &mut writer, Self::CONFIG)
            .map_err(error::Error::Encode)
            .map(|_| ())
    }
}
