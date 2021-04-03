use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

use crate::command::common_command::command::*;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this message")]
    Help,
    #[command(description = "Add user to group or create new one")]
    Start { group_id: String },
    #[command(description = "Get invite link")]
    Link,
    #[command(description = "Register user")]
    Name { username: String },
    #[command(description = "Add user to queue", parse_with = "split")]
    Push { subject: String, msg: String },
    #[command(description = "Skip first user into queue", parse_with = "split")]
    Skip { subject: String },
    #[command(description = "Show queue", parse_with = "split")]
    List { subject: String },
}

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

pub async fn answer(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx
            .answer(Command::descriptions())
            .send()
            .await
            .map(|_| ())?,
        Command::Start { group_id } => start(&cx, group_id).await?,
        Command::Link => link(&cx).await?,
        Command::Name { username } => name(&cx, username).await?,
        Command::Push { subject, msg } => push(&cx, subject, msg).await?,
        Command::Skip { subject } => skip(&cx, subject).await?,
        Command::List { subject } => list(&cx, subject).await?,
    };

    Ok(())
}
