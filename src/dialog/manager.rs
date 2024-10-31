use std::marker::PhantomData;
use std::sync::Arc;
use crate::dialog::storage::Storage;
use crate::error::LizardError;

pub struct Dialog<T, D: Storage<T> + ?Sized> {
    storage: Arc<D>,
    chat_id: String,
    user_id: String,
    _phantom: PhantomData<T>,
}

impl<T, D> Clone for Dialog<T, D>
where
    D: Storage<T> + ?Sized,
{
    fn clone(&self) -> Self {
        Dialog { storage: self.storage.clone(), chat_id: self.chat_id.clone(), user_id: self.user_id.clone(),_phantom: self._phantom }
    }
}


impl<T, D> Dialog<T, D> where
    D: Storage<T> + ?Sized, T: Send + 'static,
{
    pub fn chat_id(&self) -> &str {
        &self.chat_id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub async fn get(self) -> anyhow::Result<T, LizardError> {
        self.storage.clone().get_dialogue(self.chat_id.as_str(), self.user_id.as_str()).await
    }

    pub async fn update(self, state: T) -> anyhow::Result<(), LizardError> {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id.as_str(), self.user_id.as_str(), new_dialogue).await?;
        Ok(())
    }

    pub async fn exit(self) -> anyhow::Result<(), LizardError> {
        self.storage.clone().remove_dialogue(self.chat_id.as_str(), self.user_id.as_str()).await
    }
}

