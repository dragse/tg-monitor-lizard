use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::db::modules::{ModuleData, Settings};
use crate::db::schema::moduledata::dsl::moduledata;
use crate::db::schema::moduledata::{data_key, module_identifier};

pub fn upsert_module_settings(connection: &mut PgConnection, module_data: ModuleData) -> anyhow::Result<()> {
    diesel::insert_into(moduledata)
        .values(&module_data)
        .on_conflict((module_identifier, data_key))
        .do_update()
        .set(&module_data)
        .execute(connection)?;

    Ok(())
}

pub fn get_module_module_data(connection: &mut PgConnection, module_identifier_val: &str) -> anyhow::Result<Vec<ModuleData>> {
    let settings_data = moduledata.filter(module_identifier.eq(module_identifier_val)).select(ModuleData::as_select()).load(connection)?;

    Ok(settings_data)
}

pub fn get_module_data(connection: &mut PgConnection, module_identifier_val: &str, data_key_val: &str) -> anyhow::Result<ModuleData> {
    let data = moduledata.filter(module_identifier.eq(module_identifier_val).and(data_key.eq(data_key_val))).select(ModuleData::as_select()).first(connection)?;

    Ok(data)
}