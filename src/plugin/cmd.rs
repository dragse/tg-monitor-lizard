use std::sync::Arc;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use frankenstein::{Api, Message, SendMessageParams, TelegramApi, Update};

pub struct CommandContext {
    pub api: Arc<Api>,
    pub message: Message,
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

pub struct Command {
    pub r#use: String,
    pub short: String,
    pub run: Option<Box<dyn Fn(&CommandContext, &[String])>>,
    pub cmds: Vec<Command>,
}

impl Command {
    pub fn new(r#use: &str, short: &str, run: Option<Box<dyn Fn(&CommandContext, &[String])>>) -> Self {
        Self{
            r#use: r#use.to_owned(),
            short: short.to_owned(),
            run,
            cmds: vec![]
        }
    }

    pub fn add_cmd(&mut self, cmd: Command) {
        self.cmds.push(cmd);
    }

    pub fn execute(&self,ctx: &CommandContext,  args: &[String]) {
        if let Some(run) = self.run.as_ref() {
            run(ctx, &args[1..]);
        }

        if args.len() <= 1 {
            return;
        }

        match args[1].as_str() {
            "help" => {
                let mut msg = "Available Commands: \n".to_owned();
                for cmd in &self.cmds {
                    msg = format!("{msg}- {}: {}", cmd.r#use, cmd.short)
                }

                ctx.api.send_message(&SendMessageParams::builder().text(msg).chat_id(ctx.message.chat.id).build());
                return
            }
            _ => {}
        }

        let sub_cmd = args[1].as_str();

        for cmd in &self.cmds {
            if cmd.r#use == sub_cmd {
                return cmd.execute(ctx, &args[1..]);
            }
        }
    }
}