use crate::utils::OptArg;
use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

use crate::command::bot_admin_command::command::*;
use crate::command::common_command::command::*;
use crate::command::group_admin_command::command::*;

type OptString = OptArg<String>;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this message")]
    Help,
    #[command(description = "Add user to group or create new one")]
    Start { group_id: OptString },
    #[command(description = "Get invite link")]
    Link,
    #[command(description = "Register user")]
    Name { username: String },
    #[command(
        description = "Add user to queue",
        parse_with = "parse_command_for_push"
    )]
    Push { subject: String, msg: String },
    #[command(description = "Skip first user into queue", parse_with = "split")]
    Skip { subject: String },
    #[command(description = "Show queue", parse_with = "split")]
    List { subject: String },

    // Group admin commands
    #[command(description = "Add subject to group", parse_with = "split")]
    AddSubj { subject: String },
    #[command(description = "Remove first user in queue", parse_with = "split")]
    Pop { subject: String },
    #[command(description = "Move first record to the end", parse_with = "split")]
    Shift { subject: String, username: String },
    #[command(description = "Ban specified user", parse_with = "split")]
    Ban { username: String },
    #[command(description = "Delete group", parse_with = "split")]
    DeleteGroup { group_id: String },

    // Bot admin commands

    // #[command(description = "Add your self as bot admin", parse_with = "split")]
    // Start { subject: String },
    #[command(description = "Get list of all active groups", parse_with = "split")]
    LsGroups,
    #[command(
        description = "Get all info about specified group",
        parse_with = "split"
    )]
    LsGroup { id: String },
    #[command(description = "delete specified group", parse_with = "split")]
    RmGroup { id: String },
    #[command(description = "Add specified user to blacklist", parse_with = "split")]
    TotalBan { username: String },
}

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

pub async fn answer(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx
            .answer(Command::descriptions())
            .send()
            .await
            .map(|_| ())?,
        Command::Start { group_id } => start(&cx, group_id.into()).await?,
        Command::Link => link(&cx).await?,
        Command::Name { username } => name(&cx, username).await?,
        Command::Push { subject, msg } => push(&cx, subject, msg).await?,
        Command::Skip { subject } => skip(&cx, subject).await?,
        Command::List { subject } => list(&cx, subject).await?,
        Command::AddSubj { subject } => add_subject(&cx, subject).await?,
        Command::Pop { subject } => pop(&cx, subject).await?,
        Command::Shift { subject, username } => shift(&cx, subject, username).await?,
        Command::Ban { username } => ban(&cx, username).await?,
        Command::DeleteGroup { group_id } => delete_group(&cx, group_id).await?,
        Command::LsGroups {} => ls_groups(&cx).await?,
        Command::LsGroup { id } => ls_group(&cx, id).await?,
        Command::RmGroup { id } => rm_group(&cx, id).await?,
        Command::TotalBan { username } => total_ban(&cx, username).await?,
    };

    Ok(())
}
