use crate::utils::OptArg;
use crate::utils::opt_arg::args_parser;

use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

use crate::command::common_command::command::*;
use crate::command::group_admin_command::command::*;
use crate::command::bot_admin_command::command::*;

type OptString = OptArg<String>;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this message", parse_with = "args_parser")]
    Help,
    #[command(description = "Add user to group or create new one", parse_with = "args_parser")]
    Start { group_id: OptString },
    #[command(description = "Get invite link", parse_with = "args_parser")]
    Link,
    #[command(description = "Register user", parse_with = "args_parser")]
    Name { username: OptString },
    #[command(description = "Add user to queue", parse_with = "args_parser")]
    Push { subject: OptString, msg: OptString },
    #[command(description = "Skip first user into queue", parse_with = "args_parser")]
    Skip { subject: OptString },
    #[command(description = "Show queue", parse_with = "args_parser")]
    List { subject: OptString },

    // Group admin commands
    #[command(description = "Add subject to group", parse_with = "args_parser")]
    AddSubj { subject: OptString },
    #[command(description = "Remove first user in queue", parse_with = "args_parser")]
    Pop { subject: OptString },
    #[command(description = "Move first record to the end", parse_with = "args_parser")]
    Shift { subject: OptString, username: OptString },
    #[command(description = "Ban specified user", parse_with = "args_parser")]
    Ban { username: OptString },
    #[command(description = "Delete group", parse_with = "args_parser")]
    DeleteGroup { group_id: OptString },

    // Bot admin commands
    // #[command(description = "Add your self as bot admin", parse_with = "split")]
    // Start { subject: String },
    #[command(description = "Get list of all active groups", parse_with = "args_parser")]
    LsGroups,
    #[command(
        description = "Get all info about specified group",
        parse_with = "args_parser"
    )]
    LsGroup { id: OptString },
    #[command(description = "delete specified group", parse_with = "args_parser")]
    RmGroup { id: OptString },
    #[command(description = "Add specified user to blacklist", parse_with = "args_parser")]
    TotalBan { username: OptString },
}

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

pub async fn answer(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => get_help_msg(&cx).await?,
        Command::Start { group_id } => start(&cx, group_id.into()).await?,
        Command::Link => link(&cx).await?,
        Command::Name { username } => name(&cx, username.into()).await?,
        Command::Push { subject, msg } => push(&cx, subject.into(), msg.into()).await?,
        Command::Skip { subject } => skip(&cx, subject.into()).await?,
        Command::List { subject } => list(&cx, subject.into()).await?,
        Command::AddSubj { subject } => add_subject(&cx, subject.into()).await?,
        Command::Pop { subject } => pop(&cx, subject.into()).await?,
        Command::Shift { subject, username } => shift(&cx, subject.into(), username.into()).await?,
        Command::Ban { username } => ban(&cx, username.into()).await?,
        Command::DeleteGroup { group_id } => delete_group(&cx, group_id.into()).await?,
        Command::LsGroups {} => ls_groups(&cx).await?,
        Command::LsGroup { id } => ls_group(&cx, id.into()).await?,
        Command::RmGroup { id } => rm_group(&cx, id.into()).await?,
        Command::TotalBan { username } => total_ban(&cx, username.into()).await?,
    };

    Ok(())
}

async fn get_help_msg(cx: &Cx) -> Result<(), Box<dyn Error + Send + Sync>> {
    cx.answer(Command::descriptions()).await?;

    Ok(())
}
