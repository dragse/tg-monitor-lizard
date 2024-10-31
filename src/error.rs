use thiserror::Error;

#[derive(Error, Debug)]
pub enum LizardError {
    #[error("unknown error: {0}")]
    Unknown(String),
    #[error("cannot find the dialog for the chat {0} and user {1}")]
    UnknownDialogue(String, String),
}
