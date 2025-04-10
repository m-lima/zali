mod error;
mod model;
mod store;

pub use error::{Error, Result};
pub use store::{insert, load};
