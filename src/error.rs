use thiserror;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mathematical Operation: {0} not supported")]
    UnsupportedOperation(String),

    #[error("Invalid Reverse Polish Notation")]
    MalforedInput,

    #[error("Could not compute the result, perhaps no input was given?")]
    NoResult,

    #[error("Io Error")]
    IoError(#[from] std::io::Error),
}
