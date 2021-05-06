use crate::utils::opt_arg::args_parser;
use crate::utils::OptArg;

use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

use super::bot_admin_command::*;
use super::common_command::*;
use super::group_admin_command::*;
use crate::database::Database;
use std::sync::Arc;

type OptString = OptArg<String>;
type DB = Arc<Box<dyn Database>>;

#[derive(BotCommand)]
#[command(rename = "lowercase")]
pub enum Command {
    #[command(parse_with = "args_parser")]
    Help,
    #[command(parse_with = "args_parser")]
    Start { group_id: OptString },
    #[command(parse_with = "args_parser")]
    Link,
    #[command(parse_with = "args_parser")]
    Name { username: OptString },
    #[command(parse_with = "args_parser")]
    Push { subject: OptString, msg: OptString },
    #[command(parse_with = "args_parser")]
    Skip { subject: OptString },
    #[command(parse_with = "args_parser")]
    List { subject: OptString },

    // Group admin commands
    #[command(parse_with = "args_parser")]
    AddSubj { subject: OptString },
    #[command(parse_with = "args_parser")]
    Pop { subject: OptString },
    #[command(parse_with = "args_parser")]
    Shift {
        subject: OptString,
        username: OptString,
    },
    #[command(parse_with = "args_parser")]
    Ban { username: OptString },
    #[command(parse_with = "args_parser")]
    DeleteGroup { group_id: OptString },

    // Bot admin commands
    #[command(parse_with = "args_parser")]
    LsGroups,
    #[command(parse_with = "args_parser")]
    LsGroup { id: OptString },
    #[command(parse_with = "args_parser")]
    RmGroup { id: OptString },
    #[command(parse_with = "args_parser")]
    TotalBan { username: OptString },
}

pub type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
pub type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn answer(cx: Cx, command: Command, db: Arc<Box<dyn Database>>) -> Res {
    match command {
        Command::Help => get_help_msg(&cx, &db).await,
        Command::Start { group_id } => start(&cx, group_id.into(), &db).await,
        Command::Link => link(&cx).await,
        Command::Name { username } => name(&cx, username.into(), &db).await,
        Command::Push { subject, msg } => push(&cx, subject.into(), msg.into()).await,
        Command::Skip { subject } => skip(&cx, subject.into()).await,
        Command::List { subject } => list(&cx, subject.into()).await,
        Command::AddSubj { subject } => add_subject(&cx, subject.into()).await,
        Command::Pop { subject } => pop(&cx, subject.into(), &db).await,
        Command::Shift { subject, username } => shift(&cx, subject.into(), username.into()).await,
        Command::Ban { username } => ban(&cx, username.into()).await,
        Command::DeleteGroup { group_id } => delete_group(&cx, group_id.into()).await,
        Command::LsGroups {} => ls_groups(&cx).await,
        Command::LsGroup { id } => ls_group(&cx, id.into()).await,
        Command::RmGroup { id } => rm_group(&cx, id.into()).await,
        Command::TotalBan { username } => total_ban(&cx, username.into()).await,
    }
}

static USER_HELP_MSG: &'static str = "These commands are supported:\n\
/help - Display this message\n\
/start - Add user to group or create new one\n\
/link - Get invite link\n\
/name - Register user\n\
/push - Add user to queue\n\
/skip - Skip first user into queue\n\
/list - Show queue";

static GROUP_ADMIN_HELP_MSG: &'static str = "These commands are supported:\n\n\
User level:\n\
/help - Display this message\n\
/start - Add user to group or create new one\n\
/link - Get invite link\n\
/name - Register user\n\
/push - Add user to queue\n\
/skip - Skip first user into queue\n\
/list - Show queue\n\n\
Group admin level:\n\
/addsubj - Add subject to group\n\
/pop - Remove first user in queue\n\
/shift - Move first record to the end\n\
/ban - Ban specified user\n\
/deletegroup - Delete group";

static BOT_ADMIN_HELP_MSG: &'static str = "These commands are supported:\n\n\
User level:\n\
/help - Display this message\n\
/start - Add user to group or create new one\n\
/link - Get invite link\n\
/name - Register user\n\
/push - Add user to queue\n\
/skip - Skip first user into queue\n\
/list - Show queue\n\n\
Group admin level:\n\
/addsubj - Add subject to group\n\
/pop - Remove first user in queue\n\
/shift - Move first record to the end\n\
/ban - Ban specified user\n\
/deletegroup - Delete group\n\n\
Bot admin level:\n\
/lsgroups - Get list of all active groups\n\
/lsgroup - Get all info about specified group\n\
/rmgroup - delete specified group\n\
/totalban - Add specified user to blacklist";

async fn get_help_msg(cx: &Cx, db: &Arc<Box<dyn Database>>) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            if db.is_admin(user.id).await? {
                cx.answer(BOT_ADMIN_HELP_MSG).await?;
            } else {
                cx.answer(USER_HELP_MSG).await?;
            }
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

async fn start(cx: &Cx, group_id: Option<String>, db: &DB) -> Res {
    let group_id: Option<String> = group_id.into();

    if let Some(s) = &group_id {
        if is_admin_password(s) {
            return adm_start(&cx, &db).await;
        }
    }

    common_start(&cx, group_id, &db).await?;

    Ok(())
}
