use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvVarsError {
    #[error("Invalid environment variable format: {key}={value}")]
    InvalidFormat { key: String, value: String },

    #[error("Duplicate environment variable: {0}")]
    DuplicateVariable(String),

    #[error("Environment variable validation failed: {0}")]
    ValidationError(String),

    #[error("Failed to set environment variables: {status_code} - {message}")]
    SetError { status_code: u16, message: String },

    #[error("Missing workspace ID. Please provide via --workspace-id or $WORKSPACE_ID environment variable")]
    MissingWorkspaceId,

    #[error("API error: {status_code} - {message}")]
    ApiError { status_code: u16, message: String },
}
