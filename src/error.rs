use thiserror::Error;



#[derive(Debug,Error)]
pub enum CliError {

    #[error("Failed to clone repository: {0}")]
    CloneFailed(String),
     #[error("Failed to init repository: {0}")]
    InitFailed(String),

    #[error("unknown data store error")]
    Unknown,
}




pub type Result<T> = std::result::Result<T, CliError>;