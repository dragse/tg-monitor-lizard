use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::db::schema::settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Settings {
    pub chat_id: i64,
    pub module_identifier: String,
    pub enabled: bool,
    pub configuration: serde_json::Value,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::db::schema::moduledata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModuleData {
    pub module_identifier: String,
    pub data_key: String,
    pub data: serde_json::Value,
}