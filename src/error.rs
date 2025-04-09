#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] Args),
    #[error("Could not resolve data dir")]
    DataPath,
    #[error("Could not initialize data dir: {0}")]
    DataPathInit(std::io::Error),
    #[error("Failed to access data: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to allocate: {0}")]
    Allocation(#[from] std::collections::TryReserveError),
    #[error("Failed to get current time: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("Failed to decode: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Failed to encode: {0}")]
    Encode(#[from] bincode::error::EncodeError),
}

#[derive(Debug, thiserror::Error)]
pub enum Args {
    #[error("No action provided")]
    NoAction,
    #[error("Unknown action provided: `{0}`")]
    UnknownAction(String),
    #[error("No query provided")]
    NoQuery,
    #[error("No path provided")]
    NoPath,
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),
}
