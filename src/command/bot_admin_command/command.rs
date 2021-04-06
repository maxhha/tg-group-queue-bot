use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn adm_start(cx: &Cx, pwd: String) -> Res {
    if pwd.trim_start().is_empty() {
        cx.reply_to("Seems like you specify empty password")
            .send()
            .await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered as bot admin.", nickname))
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

pub async fn ls_groups(cx: &Cx) -> Res {
    cx.answer(format!("List of all registered groups:")).await?;

    Ok(())
}

pub async fn ls_group(cx: &Cx, group_id: String) -> Res {
    if group_id.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify group_id")
            .send()
            .await?;
        return Ok(());
    }

    cx.answer(format!("Group #{} info:", group_id)).await?;

    Ok(())
}

pub async fn rm_group(cx: &Cx, group_id: String) -> Res {
    if group_id.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify group_id")
            .send()
            .await?;
        return Ok(());
    }

    cx.answer(format!("Group #{} deleted:", group_id)).await?;

    Ok(())
}

pub async fn ban(cx: &Cx, username: String) -> Res {
    if username.trim_start().is_empty() {
        cx.reply_to("Seems like you forget to specify username")
            .send()
            .await?;
        return Ok(());
    }

    cx.answer(format!("User {} banned:", username)).await?;

    Ok(())
}
