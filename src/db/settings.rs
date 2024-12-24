use diesel::{PgConnection};
use futures::StreamExt;
use crate::db::{modules, schema};
use crate::db::schema::settings::{chat_id, configuration, enabled, module_identifier, table};
use crate::db::schema::settings::dsl::settings;
use diesel::prelude::*;
use crate::db::modules::Settings;

pub fn upsert_module_settings(connection: &mut PgConnection, settings_data: &Settings) -> anyhow::Result<()> {
    diesel::insert_into(settings)
        .values(settings_data)
        .on_conflict((chat_id, module_identifier))
        .do_update()
        .set(settings_data)
        .execute(connection)?;

    Ok(())
}

pub fn get_chat_module_settings(connection: &mut PgConnection, chat_id_val: i64) -> anyhow::Result<Vec<Settings>> {
    use self::schema::settings::dsl::*;

    let settings_data = settings.filter(chat_id.eq(chat_id_val)).select(Settings::as_select()).load(connection)?;

    Ok(settings_data)
}

pub fn get_chat_module_setting(connection: &mut PgConnection, chat_id_val: i64, module_identifier_val: &str) -> anyhow::Result<Settings> {
    use self::schema::settings::dsl::*;

    let settings_data = settings.filter(chat_id.eq(chat_id_val).and(module_identifier.eq(module_identifier_val))).select(Settings::as_select()).first(connection)?;

    Ok(settings_data)
}