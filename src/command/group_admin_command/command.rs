use crate::Database;
use std::sync::Arc;
use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;
type DB = Arc<Box<dyn Database>>;

pub async fn add_subject(cx: &Cx, subject: Option<String>, db: &DB) -> Res {
    if None == subject {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            db.add_subject(user.id, &subject.clone().unwrap()).await?;

            cx.answer(format!(
                "@{} registered new subject #{}.",
                nickname,
                subject.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn pop(cx: &Cx, subject: Option<String>) -> Res {
    if None == subject {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} popped first pos from #{} queue.",
                nickname,
                subject.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn shift(cx: &Cx, subject: Option<String>, username: Option<String>) -> Res {
    if None == subject {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            if None == username {
                cx.answer(format!(
                    "@{} skipped first pos in #{} queue.",
                    nickname,
                    subject.unwrap()
                ))
                .await?;
                return Ok(());
            }

            cx.answer(format!(
                "@{} skipped first @{} pos in #{} queue.",
                nickname,
                username.unwrap(),
                subject.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn ban(cx: &Cx, username: Option<String>) -> Res {
    if None == username {
        cx.reply_to("Seems like you forget to specify username")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!(
                "@{} banned @{} and deleted him(her) from group.",
                nickname,
                username.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn delete_group(cx: &Cx, group_id: Option<String>) -> Res {
    if None == group_id {
        cx.reply_to("Seems like you forget to specify group_id")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!(
                "@{} deleted #{} group.",
                nickname,
                group_id.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}
