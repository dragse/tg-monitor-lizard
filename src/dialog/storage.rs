mod memory;

use std::sync::Arc;
use crate::error::LizardError;

pub trait Storage<D> {
    async fn remove_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
    ) -> anyhow::Result<(), LizardError>
    where
        D: Send + 'static;

    async fn update_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
        dialogue: D,
    ) -> anyhow::Result<(), LizardError>
    where
        D: Send + 'static;

    async fn get_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
    ) -> anyhow::Result<D, LizardError>;
}
