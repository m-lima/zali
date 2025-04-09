use crate::error;

type Result<T = ()> = std::result::Result<T, error::Data>;

const DIRNAME: &str = env!("CARGO_PKG_NAME");
const FILENAME: &str = "entries";

pub struct Data<const WRITE: bool>(std::fs::File);

fn path() -> Result<std::path::PathBuf> {
    dirs::data_local_dir()
        .map(|p| p.join(DIRNAME))
        .ok_or(error::Data::Resolve)
}

impl Data<false> {
    pub fn new() -> Result<Self> {
        let path = path().map(|p| p.join(FILENAME))?;

        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(error::Data::Open)?;

        fs2::FileExt::lock_shared(&file).map_err(error::Data::Lock)?;

        Ok(Self(file))
    }
}

impl Data<true> {
    pub fn new() -> Result<Self> {
        let data_dir = path()?;
        let data_path = data_dir.join(FILENAME);

        if !data_path.exists() {
            std::fs::create_dir(data_dir).map_err(error::Data::Init)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(data_path)
            .map_err(error::Data::Open)?;

        fs2::FileExt::lock_exclusive(&file).map_err(error::Data::Lock)?;

        Ok(Self(file))
    }

    pub fn writer(self) -> std::io::BufWriter<std::fs::File> {
        std::io::BufWriter::new(self.0)
    }
}

impl<const WRITE: bool> Data<WRITE> {
    pub fn reader(&self) -> impl std::io::BufRead {
        std::io::BufReader::new(&self.0)
    }
}
