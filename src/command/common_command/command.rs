use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn start(cx: &Cx, group_id: Option<String>) -> Res {
    if None == group_id {
        cx.reply_to("Seems like you forget to specify group_id")
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!(
                "@{} registered new group #{}.",
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
