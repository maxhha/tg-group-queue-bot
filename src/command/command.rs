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

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

pub async fn answer(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await.map(|_| ())?,
        Command::Start { group_id } => start(&cx, group_id).await?,
        Command::Link => link(&cx).await?,
        Command::Name { username } => name(&cx, username).await?,
        Command::Push { subject, msg } => push(&cx, subject, msg).await?,
        Command::Skip { subject } => skip(&cx, subject).await?,
        Command::List { subject } => list(&cx, subject).await?,
    };

    Ok(())
}

async fn start(cx: &Cx, group_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if group_id.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot group_id").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} register new group #{}.", nickname, group_id)).await?;
        }
        None => {
            cx.reply_to("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn link(cx: &Cx) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("Your invite link is : {}", "")).send().await?;
        }
        None => {
            cx.reply_to("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn name(cx: &Cx, username: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered as {}.", nickname, username)).send().await?;
        }
        None => {
            cx.reply_to("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn push(cx: &Cx, subject: String, msg: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot subject").send().await?;

        return Ok(());
    }

    if msg.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot message").send().await?;

        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} pushed to {} queue with msg {}.", nickname, subject, msg))
                .await?;
        }
        None => {
            cx.reply_to("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn skip(cx: &Cx, subject: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot subject").send().await?;

        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} skipped {} queue.", nickname, subject)).await?;
        }
        None => {
            cx.reply_to("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn list(cx: &Cx, subject: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    if subject.trim_start().is_empty() {
        cx.reply_to("All active queues:").send().await?;

        return Ok(());
    }

    cx.answer(format!("Queue for {} is.", subject)).await?;

    Ok(())
}