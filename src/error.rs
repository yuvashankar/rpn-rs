use thiserror;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mathematical Operation: {0} not suppported")]
    UnsupportedOperation(String),

    #[error("Invalid Reverse Polish Notation")]
    MalforedInput,

    #[error("Could not compute the result, perhaps no input was given?")]
    NoResult,
}
