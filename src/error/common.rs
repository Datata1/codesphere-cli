use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

use super::endpoints::Error;
