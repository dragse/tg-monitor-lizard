use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum JoinValidation {
    #[default]
    Disabled,
    InlineKeyboardButtonMath{
        question_size: i32,
        allow_retry: bool
    },
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GroupConfiguration {
    pub join_message: String,
    pub join_validation: JoinValidation,
    pub leave_message: String
}

impl Into<Vec<u8>> for GroupConfiguration {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl From<Vec<u8>> for GroupConfiguration {
    fn from(value: Vec<u8>) -> Self {
        serde_json::from_slice(value.as_slice()).unwrap()
    }
}

impl From<&[u8]> for GroupConfiguration {
    fn from(value: &[u8]) -> Self {
        serde_json::from_slice(value).unwrap()
    }
}
