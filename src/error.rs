#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] Args),
    #[error(transparent)]
    Access(#[from] crate::entry::Error),
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

// #[derive(Debug, thiserror::Error)]
// pub enum Access {
//     #[error(transparent)]
//     Data(#[from] Data),
//     #[error(transparent)]
//     Entry(#[from] Entry),
// }
