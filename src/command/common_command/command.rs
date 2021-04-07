use teloxide::prelude::*;
use teloxide::utils::command::ParseError;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn start(cx: &Cx, group_id: Option<String>) -> Res {
    if let Some(group_id) = group_id {
        match cx.update.from() {
            Some(user) => {
                let nickname = user.clone().username.expect("Must be user");
                cx.answer(format!("@{} registered new group #{}.", nickname, group_id))
                    .await?;
            }
            None => {
                cx.answer("Use this command as common message").await?;
            }
        }
    } else {
        cx.reply_to("Seems like you forget to specify group_id")
            .await?;
    }

    Ok(())
}

pub async fn link(cx: &Cx) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("Your invite link is : {}", "")).await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn name(cx: &Cx, username: String) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered as {}.", nickname, username))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub fn parse_command_for_push(s: String) -> Result<(String, String), ParseError> {
    let s = s.trim();

    let space_idx = s.find(' ').ok_or(ParseError::IncorrectFormat(
        "must be at least 2 arguments".into(),
    ))?;

    let (subject, rest) = s.split_at(space_idx);
    let message = rest.trim_start();

    Ok((subject.into(), message.into()))
}

pub async fn push(cx: &Cx, subject: String, msg: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;

        return Ok(());
    }

    if msg.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot message").await?;

        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} pushed to {} queue with msg {}.",
                nickname, subject, msg
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn skip(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;

        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} skipped {} queue.", nickname, subject))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn list(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("All active queues:").await?;

        return Ok(());
    }

    cx.answer(format!("Queue for {} is.", subject)).await?;

    Ok(())
}
