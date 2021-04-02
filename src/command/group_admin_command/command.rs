use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn add_subject(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.answer("Seems like you forgot to specify subject").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered new subject #{}.", nickname, subject)).await?;
        }
        None => {
            cx.answer("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

pub async fn pop(cx: &Cx, subject: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.answer("Seems like you forgot to specify subject").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} popped first pos from #{} queue.", nickname, subject)).await?;
        }
        None => {
            cx.answer("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

async fn skip(cx: &Cx, subject: String, username: String) -> Res {
    if subject.trim_start().is_empty() {
        cx.answer("Seems like you forgot to specify subject").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            if username.trim_start().is_empty() {
                cx.answer(format!("@{} skipped first pos in #{} queue.", nickname, subject)).await?;
                return Ok(());
            }

            cx.answer(format!("@{} skipped first @{} pos in #{} queue.", nickname, username, subject))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

pub async fn ban(cx: &Cx, username: String) -> Res {
    if username.trim_start().is_empty() {
        cx.answer("Seems like you forgot to specify username").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!("@{} banned @{} and deleted him(her) from group.", nickname, username))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").send().await?;
        }
    }

    Ok(())
}

pub async fn delete_group(cx: &Cx, group_id: String) -> Res {
    if group_id.trim_start().is_empty() {
        cx.answer("Seems like you forgot to specify group_id").send().await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            cx.answer(format!("@{} deleted #{} group.", nickname, group_id))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").send().await?;
        }
    }

    Ok(())
}
