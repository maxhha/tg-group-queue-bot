use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

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

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Start { group_id } => {
            cx.answer(format!("Bot started for {}.", group_id)).await?
        }
        Command::Link => cx.answer(format!("Your invite link is : {}", "")).send().await?,
        Command::Name { username } => {
            cx.answer(format!("User {} registered.", username)).await?
        }
        Command::Push { subject, msg } => {
            cx.answer(format!("Successfully added for {} with msg {}.", subject, msg))
                .await?
        }
        Command::Skip { subject } => {
            cx.answer(format!("Successfully skipped for {}.", subject))
                .await?
        }
        Command::List { subject } => {
            cx.answer(format!("Queue for {} is.", subject))
                .await?
        }
    };

    Ok(())
}