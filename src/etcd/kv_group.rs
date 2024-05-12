use crate::etcd::client::ETCD_CLIENT;
use crate::model;

pub async fn save_group_validation(
    group_id: i64,
    group_configuration: model::GroupConfiguration,
) -> anyhow::Result<()> {
    let group_value: Vec<u8> = group_configuration.into();
    let mut etcd = ETCD_CLIENT.get().await.clone();

    etcd.client
        .put(
            format!(
                "/tg_monitor_lizard/{}/group/configurations/{}",
                etcd.tenant.as_str(),
                group_id
            ),
            group_value,
            None,
        )
        .await?;

    Ok(())
}
pub async fn delete_group_validation(group_id: i64) -> anyhow::Result<()> {
    let mut etcd = ETCD_CLIENT.get().await.clone();

    let path = format!(
        "/tg_monitor_lizard/{}/group/configurations/{}",
        etcd.tenant.as_str(),
        group_id
    );

    etcd.delete_item(path.as_str()).await
}

pub async fn get_group_validations() -> anyhow::Result<Vec<model::GroupConfiguration>> {
    let mut etcd = ETCD_CLIENT.get().await.clone();

    let path = format!("/dragons_realm//configurations/{}", etcd.tenant);
    etcd.get_items(path.as_str()).await
}

pub async fn get_group_validation(hoard_id: i64) -> anyhow::Result<model::GroupConfiguration> {
    let mut etcd = ETCD_CLIENT.get().await.clone();

    let path = format!(
        "/dragons_realm/{}//configurations/{}",
        etcd.tenant, hoard_id
    );
    etcd.get_item(path.as_str()).await
}
