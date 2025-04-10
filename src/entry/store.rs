use super::{Error, Result, model};

const DIRNAME: &str = env!("CARGO_PKG_NAME");
const FILENAME: &str = "entries";
const TEMP_FILENAME: &str = "_entries";
const CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
> = bincode::config::standard().with_fixed_int_encoding();

fn path() -> Result<std::path::PathBuf> {
    dirs::data_local_dir()
        .map(|p| p.join(DIRNAME))
        .ok_or(Error::Resolve)
}

pub fn load() -> Result<Vec<model::Entry>> {
    let path = path().map(|p| p.join(FILENAME))?;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(Error::Open)?;

    fs2::FileExt::lock_shared(&file).map_err(Error::Lock)?;
    let reader = std::io::BufReader::new(file);

    bincode::decode_from_reader(reader, CONFIG).map_err(Error::Decode)
}

pub fn insert(access: String) -> Result {
    let now = model::LastVisit::now()?;
    let access = model::Access::try_from(access)?;
    let entry = model::Entry::new(access, now);

    let dir = path()?;
    let store_path = dir.join(FILENAME);

    if !store_path.exists() {
        std::fs::create_dir(&dir).map_err(Error::Init)?;
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&store_path)
        .map_err(Error::Open)?;

    fs2::FileExt::lock_exclusive(&file).map_err(Error::Lock)?;

    let reader = std::io::BufReader::new(&file);
    let mut entries = bincode::decode_from_reader::<Vec<model::Entry>, _, _>(reader, CONFIG)
        .ok()
        .unwrap_or_default();

    eprintln!("Loaded   {entries:?}");

    entries.retain(|e| e.days_since(now) < 365);
    eprintln!("Retained {entries:?}");

    #[cfg(feature = "check_on_load")]
    {
        entries.sort_unstable();
        eprintln!("Sorted   {entries:?}");
        entries.dedup_by(|a, b| a.path == b.path);
        eprintln!("Deduped  {entries:?}");
    }

    match entries.binary_search_by(|other| other.path.cmp(&entry.path)) {
        Ok(idx) => unsafe { *entries.get_unchecked_mut(idx) = entry },
        Err(idx) => entries.insert(idx, entry),
    }
    eprintln!("Pushed   {entries:?}");

    let temp_path = dir.join(TEMP_FILENAME);

    let file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp_path)
        .map_err(Error::Write)?;

    let mut writer = std::io::BufWriter::new(file);
    bincode::encode_into_std_write(entries, &mut writer, CONFIG).map_err(Error::Encode)?;
    let result = std::fs::rename(&temp_path, store_path).map_err(Error::Write);

    let _ = std::fs::remove_file(temp_path);

    result
}
