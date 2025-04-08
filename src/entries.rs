use crate::{Result, error};

fn data_path() -> Result<&'static std::path::PathBuf> {
    static PATH: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        dirs::data_local_dir().map(|p| p.join(env!("CARGO_PKG_NAME")).join("entries"))
    })
    .as_ref()
    .ok_or(error::Error::DataPath)
}

#[derive(Debug)]
pub struct Entry {
    score: u8,
    path: std::path::PathBuf,
}

impl Entry {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut bytes = bytes.into_iter().copied();
        let score = bytes.next()?;
        let path_bytes = bytes.take_while(|&b| b > 0).collect::<Vec<_>>();

        if path_bytes.is_empty() {
            None
        } else {
            let path = std::path::PathBuf::from(
                <std::ffi::OsString as std::os::unix::ffi::OsStringExt>::from_vec(path_bytes),
            );
            Some(Self { score, path })
        }
    }
}

#[derive(Debug)]
pub struct EntryView<'a> {
    score: u8,
    path: &'a std::path::Path,
}

impl<'a> EntryView<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Option<Self> {
        let score = bytes.get(0).copied()?;
        let end = bytes
            .iter()
            .skip(1)
            .position(|&b| b == 0)
            .unwrap_or(bytes.len());
        if end > 0 {
            None
        } else {
            let path_bytes =
                <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(&bytes[1..end + 1]);
            let path = std::path::Path::new(path_bytes);
            Some(Self { score, path })
        }
    }
}

pub struct Access(std::path::PathBuf);

impl Access {
    pub fn register(self) -> Result {
        // let path = data_path()?;
        // let bytes = {
        //     let mut file = std::fs::OpenOptions::new()
        //         .read(true)
        //         .write(true)
        //         .create(true)
        //         .open(path)?;
        //
        //     fs2::FileExt::lock_exclusive(&file);
        //
        //     let size = file.metadata().map(|m| m.len() as usize).ok();
        //     let mut bytes = Vec::new();
        //     bytes.try_reserve_exact(size.unwrap_or(0))?;
        //     std::io::Read::read_to_end(&mut file, &mut bytes);
        //     bytes
        // };
        todo!()
    }
}

impl TryFrom<String> for Access {
    type Error = error::Error;

    fn try_from(value: String) -> Result<Self> {
        let path = std::path::PathBuf::from(value);

        let path = path
            .canonicalize()
            .map_err(|_| error::Args::InvalidPath(path))?;

        path.as_os_str().as_encoded_bytes();

        if path.is_dir() {
            Ok(Self(path))
        } else {
            Err(error::Error::Args(error::Args::InvalidPath(path)))
        }
    }
}

pub struct Entries(Vec<u8>);

impl Entries {
    pub fn new() -> Result<Self> {
        let path = data_path()?;
        let bytes = {
            let mut file = std::fs::OpenOptions::new().read(true).open(path)?;

            fs2::FileExt::lock_shared(&file);

            let size = file.metadata().map(|m| m.len() as usize).ok();
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(size.unwrap_or(0))?;
            std::io::Read::read_to_end(&mut file, &mut bytes);
            bytes
        };

        Ok(Self(bytes))
    }
}
