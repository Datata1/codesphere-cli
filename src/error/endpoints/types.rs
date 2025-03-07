use super::env_vars::EnvVarsError;
use crate::error::common::CommonError;
use reqwest;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Common(#[from] CommonError),

    #[error(transparent)]
    EnvVars(#[from] EnvVarsError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),
}
