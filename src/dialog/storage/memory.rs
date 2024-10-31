use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::dialog::storage::Storage;
use crate::error::LizardError;

/// A dialogue storage based on [`std::collections::HashMap`].
#[derive(Debug)]
pub struct InMemStorage<D> {
    map: Mutex<HashMap<String, D>>,
}

impl<S> InMemStorage<S> {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { map: Mutex::new(HashMap::new()) })
    }

    fn key_generator(chat_id: &str, user_id: &str) -> String {
        format!("{chat_id}_{user_id}")
    }
}

impl<D> Storage<D> for InMemStorage<D>
where
    D: Clone,
    D: Send + 'static,
{

    async fn remove_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
    ) -> anyhow::Result<(), LizardError>
    where
        D: Send + 'static,
    {
        self.map.lock().await
            .remove(&InMemStorage::<D>::key_generator(chat_id, user_id))
            .map_or(Ok(()) ,|_| Ok(()))
    }

    async fn update_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
        dialogue: D,
    ) -> anyhow::Result<(), LizardError>
    where
        D: Send + 'static,
    {
        self.map.lock().await
            .insert(InMemStorage::<D>::key_generator(chat_id, user_id), dialogue)
            .map_or(Ok(()), |_| Ok(()))
    }

    async fn get_dialogue(
        self: Arc<Self>,
        chat_id: &str,
        user_id: &str,
    ) -> anyhow::Result<D, LizardError> {
        self.map.lock().await
            .get(&InMemStorage::<D>::key_generator(chat_id, user_id))
            .map_or(Err(LizardError::UnknownDialogue(chat_id.to_owned(), user_id.to_owned())), |dialog| Ok(dialog.clone()))
    }
}