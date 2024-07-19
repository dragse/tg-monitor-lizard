use thiserror::Error;

#[derive(Error, Debug)]
pub enum LizardError {
    #[error("unknown error: {0}")]
    Unknown(String)
}