use crate::error;

type Result<T = ()> = std::result::Result<T, error::Data>;

const DIRNAME: &str = env!("CARGO_PKG_NAME");
const FILENAME: &str = "entries";
const TEMP_FILENAME: &str = "_entries";

fn path() -> Result<std::path::PathBuf> {
    dirs::data_local_dir()
        .map(|p| p.join(DIRNAME))
        .ok_or(error::Data::Resolve)
}

pub struct Loader(std::fs::File);

impl Loader {
    pub fn new() -> Result<Self> {
        let path = path().map(|p| p.join(FILENAME))?;

        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(error::Data::Open)?;

        fs2::FileExt::lock_shared(&file).map_err(error::Data::Lock)?;

        Ok(Self(file))
    }

    pub fn reader(self) -> impl std::io::BufRead {
        std::io::BufReader::new(self.0)
    }
}

pub struct Writer<const WRITEABLE: bool> {
    data_dir: std::path::PathBuf,
    file: std::fs::File,
}

impl Writer<false> {
    pub fn new() -> Result<Self> {
        let data_dir = path()?;
        let data_path = data_dir.join(FILENAME);

        if !data_path.exists() {
            std::fs::create_dir(&data_dir).map_err(error::Data::Init)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(data_path)
            .map_err(error::Data::Open)?;

        fs2::FileExt::lock_exclusive(&file).map_err(error::Data::Lock)?;

        Ok(Self { data_dir, file })
    }

    pub fn read<T, F: FnOnce(std::io::BufReader<&std::fs::File>) -> T>(
        self,
        f: F,
    ) -> (Writer<true>, T) {
        let result = f(std::io::BufReader::new(&self.file));
        (
            Writer {
                data_dir: self.data_dir,
                file: self.file,
            },
            result,
        )
    }
}

impl Writer<true> {
    pub fn write<
        T,
        E,
        F: FnOnce(std::io::BufWriter<std::fs::File>) -> std::result::Result<T, E>,
    >(
        self,
        f: F,
    ) -> std::result::Result<Result, E> {
        let temp_path = self.data_dir.join(TEMP_FILENAME);
        let data_path = self.data_dir.join(FILENAME);

        let file = match std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp_path)
            .map_err(error::Data::Write)
        {
            Ok(file) => file,
            Err(err) => return Ok(Err(err)),
        };

        f(std::io::BufWriter::new(file))
            .map(|_| std::fs::rename(temp_path, data_path).map_err(error::Data::Write))
    }
}
