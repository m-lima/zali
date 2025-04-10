#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] Args),
    #[error(transparent)]
    Entry(#[from] crate::entry::Error),
    #[error(transparent)]
    Query(#[from] crate::action::query::Error),
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
