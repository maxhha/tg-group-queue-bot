use teloxide::prelude::*;

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
                cx.answer("Use this command as common message")
                    .send()
                    .await?;
            }
        }
    } else {
        cx.reply_to("Seems like you forget to specify group_id")
            .send()
            .await?;
    }

    Ok(())
}

pub async fn link(cx: &Cx) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("Your invite link is : {}", ""))
                .send()
                .await?;
        }
        None => {
            cx.answer("Use this command as common message")
                .send()
                .await?;
        }
    }

    Ok(())
}

pub async fn name(cx: &Cx, username: String) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered as {}.", nickname, username))
                .send()
                .await?;
        }
        None => {
            cx.answer("Use this command as common message")
                .send()
                .await?;
        }
    }

    Ok(())
}

pub async fn push(cx: &Cx, subject: String, msg: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject")
            .send()
            .await?;

        return Ok(());
    }

    if msg.trim_start().is_empty() {
        cx.reply_to("Seems like you forgot message").send().await?;

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
            cx.answer("Use this command as common message")
                .send()
                .await?;
        }
    }

    Ok(())
}

pub async fn skip(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify subject")
            .send()
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
            cx.answer("Use this command as common message")
                .send()
                .await?;
        }
    }

    Ok(())
}

pub async fn list(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.reply_to("All active queues:").send().await?;

        return Ok(());
    }

    cx.answer(format!("Queue for {} is.", subject)).await?;

    Ok(())
}
