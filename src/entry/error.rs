pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to resolve data dir")]
    Resolve,
    #[error("Failed to open entries: {0}")]
    Open(std::io::Error),
    #[error("Failed to write entries: {0}")]
    Write(std::io::Error),
    #[error("Failed to acquire file lock: {0}")]
    Lock(std::io::Error),
    #[error("Failed to initialize data dir: {0}")]
    Init(std::io::Error),
    #[error("Failed to encode: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("Failed to decode: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Failed to get current time: {0}")]
    SystemTime(std::time::SystemTimeError),
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),
    #[error("Path is not a directory: {0:?}")]
    NotDir(std::path::PathBuf),
}
