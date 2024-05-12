use std::env;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ETCD_CLIENT: AsyncOnce<EtcdClient> = AsyncOnce::new(EtcdClient::new(
        "default",
        env::var("DATABASE_ADDRESSES")
            .expect("'DATABASE_ADDRESSES' is required")
            .split(",")
            .map(|addr| addr.to_owned())
            .collect()
    ));
}

#[derive(Clone)]
pub struct EtcdClient {
    pub tenant: String,
    pub client: etcd_client::Client,
}

impl EtcdClient {
    pub async fn new(tenant: &str, endpoints: Vec<String>) -> Self {
        let client = etcd_client::Client::connect(endpoints, None)
            .await
            .expect("Can't connect to database!");

        Self {
            tenant: tenant.to_owned(),
            client,
        }
    }
}
