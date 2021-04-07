use teloxide::prelude::*;

use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn adm_start(cx: &Cx, pwd: Option<String>) -> Res {
    if None == pwd {
        cx.reply_to("Seems like you specify empty password").await?;
        return Ok(());
    }

    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");
            cx.answer(format!("@{} registered as bot admin.", nickname))
                .await?;
        }
        None => {
            cx.answer("Use this command as common message").await?;
        }
    }

    Ok(())
}

pub async fn ls_groups(cx: &Cx) -> Res {
    cx.answer(format!("List of all registered groups:")).await?;
    Ok(())
}

pub async fn ls_group(cx: &Cx, group_id: Option<String>) -> Res {
    if None == group_id {
        cx.reply_to("Seems like you forget to specify group_id")
            .await?;
        return Ok(());
    }

    cx.answer(format!("Group #{} info:", group_id.unwrap()))
        .await?;

    Ok(())
}

pub async fn rm_group(cx: &Cx, group_id: Option<String>) -> Res {
    if None == group_id {
        cx.reply_to("Seems like you forget to specify group_id")
            .await?;
        return Ok(());
    }

    cx.answer(format!("Group #{} deleted:", group_id.unwrap()))
        .await?;

    Ok(())
}

pub async fn total_ban(cx: &Cx, username: Option<String>) -> Res {
    if None == username {
        cx.reply_to("Seems like you forget to specify username")
            .await?;
        return Ok(());
    }

    cx.answer(format!("User {} banned:", username.unwrap()))
        .await?;

    Ok(())
}
