#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] Args),
    #[error(transparent)]
    Access(#[from] Access),
}

#[derive(Debug, thiserror::Error)]
pub enum Args {
    #[error("No action provided")]
    NoAction,
    #[error("Unknown action provided: `{0}`")]
    UnknownAction(String),
    #[error("Missing expected argument")]
    Missing,
}

#[derive(Debug, thiserror::Error)]
pub enum Access {
    #[error(transparent)]
    Data(#[from] Data),
    #[error(transparent)]
    Entry(#[from] Entry),
}

#[derive(Debug, thiserror::Error)]
pub enum Data {
    #[error("Failed to resolve data dir")]
    Resolve,
    #[error("Failed to open entries")]
    Open(std::io::Error),
    #[error("Failed to acquire file lock")]
    Lock(std::io::Error),
    #[error("Failed to initialize data dir: {0}")]
    Init(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum Entry {
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),
    #[error("Path is not a directory: {0:?}")]
    NotDir(std::path::PathBuf),
    #[error("Failed to get current time: {0}")]
    SystemTime(std::time::SystemTimeError),
    #[error("Failed to encode: {0}")]
    Encode(#[from] bincode::error::EncodeError),
}
