#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] Args),
    #[error("Could not establish data dir")]
    DataPath,
    #[error("Failed to access data: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to allocate: {0}")]
    Allocation(#[from] std::collections::TryReserveError),
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
