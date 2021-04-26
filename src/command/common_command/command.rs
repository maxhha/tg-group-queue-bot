use crate::Database;
use std::sync::Arc;
use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;
type DB = Arc<Box<dyn Database>>;

pub async fn common_start(cx: &Cx, group_id: Option<String>, db: &DB) -> Res {
    if let Some(user) = cx.update.from() {
        let nickname = user.clone().username.expect("Must be user");

        if let Some(_) = db.find_group(user.id).await? {
            cx.reply_to("Sorry, you cant enter new room while membering in another.")
                .await?;
        } else if let Some(group_id) = group_id {
            if db.get_group(&group_id).await?.is_some() {
                db.add_group_member(&group_id, user.id).await?;
                cx.answer(format!("@{} entered group #{}.", nickname, group_id))
                    .await?;
            } else {
                cx.answer("Cant find room").await?;
            }
        } else {
            let group_id = db.create_group(user.id).await?;
            cx.answer(format!("@{} registered new group #{}.", nickname, group_id))
                .await?;
        }
    } else {
        cx.answer("Use this command as common message").await?;
    }

    Ok(())
}

pub async fn link(cx: &Cx) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("Your invite link is : {}", "")).await?;
        }
        None => {}
    }

    Ok(())
}

pub async fn name(cx: &Cx, username: Option<String>) -> Res {
    if None == username {
        cx.reply_to("Seems like you forget to specify username")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} registered as {}.",
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

pub async fn push(cx: &Cx, subject: Option<String>, msg: Option<String>) -> Res {
    if None == subject {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;
        return Ok(());
    }

    if None == msg {
        cx.reply_to("Seems like you forgot message").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} pushed to #{} queue with msg {}.",
                nickname,
                subject.unwrap(),
                msg.unwrap()
            ))
            .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn skip(cx: &Cx, subject: Option<String>) -> Res {
    if None == subject {
        cx.reply_to("Seems like you forget to specify subject")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} skipped #{} queue.",
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

pub async fn list(cx: &Cx, subject: Option<String>) -> Res {
    if None == subject {
        cx.reply_to("All active queues:").await?;
        return Ok(());
    }

    cx.answer(format!("Queue for {} is.", subject.unwrap()))
        .await?;

    Ok(())
}
