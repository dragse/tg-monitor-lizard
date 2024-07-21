use teloxide::RequestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LizardError {
    #[error("unknown error: {0}")]
    Unknown(String)
}

impl Into<RequestError> for LizardError {
    fn into(self) -> RequestError {
        RequestError::
    }
}