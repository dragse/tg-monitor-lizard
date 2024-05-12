use anyhow::anyhow;
use etcd_client::GetOptions;

use crate::etcd::EtcdClient;

impl EtcdClient {
    pub async fn save_item<T: Into<Vec<u8>>>(&mut self, path: &str, item: T) -> anyhow::Result<()> {
        let item_value: Vec<u8> = item.into();
        self.client.put(path, item_value, None).await?;

        Ok(())
    }

    pub async fn delete_item(&mut self, path: &str) -> anyhow::Result<()> {
        self.client.delete(path, None).await?;
        Ok(())
    }

    pub async fn get_items<T: From<Vec<u8>>>(&mut self, path: &str) -> anyhow::Result<Vec<T>> {
        let mut items = vec![];
        let result = self
            .client
            .get(path, Some(GetOptions::new().with_prefix()))
            .await?;

        for kv in result.kvs() {
            let item = kv
                .value()
                .into_iter()
                .map(|val| val.to_owned())
                .collect::<Vec<u8>>()
                .into();
            items.push(item);
        }

        Ok(items)
    }

    pub async fn get_item<T: From<Vec<u8>>>(&mut self, path: &str) -> anyhow::Result<T> {
        let resp = self.client.get(path, None).await?;
        if let Some(kv) = resp.kvs().first() {
            let item = kv
                .value()
                .into_iter()
                .map(|val| val.to_owned())
                .collect::<Vec<u8>>()
                .into();

            Ok(item)
        } else {
            Err(anyhow!("item with path ({path}) not found"))
        }
    }
}
