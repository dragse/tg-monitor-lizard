// @generated automatically by Diesel CLI.

diesel::table! {
    moduledata (module_identifier, data_key) {
        module_identifier -> Text,
        data_key -> Text,
        data -> Jsonb,
    }
}

diesel::table! {
    settings (chat_id, module_identifier) {
        chat_id -> Int8,
        module_identifier -> Text,
        enabled -> Bool,
        configuration -> Jsonb,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    moduledata,
    settings,
);
