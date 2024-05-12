use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum JoinValidation {
    Disabled,
    InlineKeyboardButtonMath,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GroupConfiguration {
    pub join_validation: JoinValidation,
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
