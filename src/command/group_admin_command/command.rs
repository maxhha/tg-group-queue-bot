use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn add_subject(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} registered new subject #{}.",
                nickname, subject
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn pop(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} popped first pos from #{} queue.",
                nickname, subject
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn shift(cx: &Cx, subject: String, username: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            if username.trim_start().is_empty() {
                cx.answer(format!(
                    "@{} skipped first pos in #{} queue.",
                    nickname, subject
                ))
                .await?;
                return Ok(());
            }

            cx.answer(format!(
                "@{} skipped first @{} pos in #{} queue.",
                nickname, username, subject
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn total_ban(cx: &Cx, username: String) -> Res {
    if username.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify username").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!(
                "@{} banned @{} and deleted him(her) from group.",
                nickname, username
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn delete_group(cx: &Cx, group_id: String) -> Res {
    if group_id.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify group_id").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!("@{} deleted #{} group.", nickname, group_id))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}
