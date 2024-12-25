use frankenstein::{ParseMode, SendMessageParams, TelegramApi};
use frankenstein::BotCommandScope::Default;
use log::error;
use serde_json::Value;
use crate::db::{get_chat_module_setting, upsert_module_settings, Settings};
use crate::plugin::Command;

pub fn get_module_cmd() -> Command {
    let mut cmd = Command::new("module", "Settings about modules", None);
    cmd.add_cmd(get_module_ls_cmd());
    cmd.add_cmd(get_module_enabled_cmd());
    cmd.add_cmd(get_module_disable_cmd());

    cmd
}

fn get_module_ls_cmd() -> Command {
    Command::new("ls", "List all existing Modules", Some(Box::new(|ctx, args| {
        let metadata = ctx.plugin_manager.get_plugins();
        let mut msg = "Available Commands: \n".to_owned();

        for meta in metadata {
            msg = format!("{msg}\\- `{}` {} \\- {}\n", meta.key, meta.name, meta.description);
        }

        let result = ctx.api.send_message(&SendMessageParams::builder().text(msg).parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());

        if let Err(why) = result {
            println!("Error sending message: {:?}", why);
        }
    })))
}

fn get_module_enabled_cmd() -> Command {
    Command::new("enable", "Enable an given Module", Some(Box::new(|ctx, args| {
        if args.is_empty() {
            ctx.api.send_message(&SendMessageParams::builder().text("The Module-Key is an required parameter").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
            return
        }

        let module_key = args.first().unwrap();

        let metadata = ctx.plugin_manager.get_plugins();
        let found = metadata.iter().find(|x| x.key == *module_key);

        if let None = found {
            ctx.api.send_message(&SendMessageParams::builder().text("The given Module-Key wasn't found in existing modules").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
            return
        }

        let connection = ctx.pool.get();

        if let Err(why) = connection {
            error!("Error getting connnection: {:?}", why);
            return
        }
        let mut connection = connection.unwrap();

        let setting_result = get_chat_module_setting(&mut connection, ctx.message.chat.id, module_key);
        
        if let Ok(setting) = setting_result {
            upsert_module_settings(&mut connection, &Settings {
                chat_id: setting.chat_id,
                module_identifier: setting.module_identifier,
                enabled: true,
                configuration: setting.configuration,
            });
        } else {
            upsert_module_settings(&mut connection, &Settings {
                chat_id:  ctx.message.chat.id,
                module_identifier: module_key.to_string(),
                enabled: true,
                configuration: Value::String("".to_string()),
            });
        }
        ctx.api.send_message(&SendMessageParams::builder().text("Module enabled successful").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
    })))
}


fn get_module_disable_cmd() -> Command {
    Command::new("disable", "Disable an given Module", Some(Box::new(|ctx, args| {
        if args.is_empty() {
            ctx.api.send_message(&SendMessageParams::builder().text("The Module-Key is an required parameter").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
            return
        }

        let module_key = args.first().unwrap();

        let metadata = ctx.plugin_manager.get_plugins();
        let found = metadata.iter().find(|x| x.key == *module_key);

        if let None = found {
            ctx.api.send_message(&SendMessageParams::builder().text("The given Module-Key wasn't found in existing modules").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
            return
        }

        let connection = ctx.pool.get();

        if let Err(why) = connection {
            error!("Error getting connnection: {:?}", why);
            return
        }
        let mut connection = connection.unwrap();

        let setting_result = get_chat_module_setting(&mut connection, ctx.message.chat.id, module_key);

        if let Ok(setting) = setting_result {
            upsert_module_settings(&mut connection, &Settings {
                chat_id: setting.chat_id,
                module_identifier: setting.module_identifier,
                enabled: false,
                configuration: setting.configuration,
            });
        } else {
            upsert_module_settings(&mut connection, &Settings {
                chat_id:  ctx.message.chat.id,
                module_identifier: module_key.to_string(),
                enabled: false,
                configuration: Value::String("".to_string()),
            });
        }
        ctx.api.send_message(&SendMessageParams::builder().text("Module disabled successful").parse_mode(ParseMode::MarkdownV2).chat_id(ctx.message.chat.id).build());
    })))
}