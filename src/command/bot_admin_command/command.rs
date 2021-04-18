use crate::Database;
use std::sync::Arc;
use teloxide::prelude::*;

use data_encoding::BASE64;
use ring::digest::{digest, SHA512};
use std::error::Error;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
type Res = Result<(), Box<dyn Error + Send + Sync>>;
type DB = Arc<Box<dyn Database>>;

pub fn is_admin_password(pass: &String) -> bool {
    let hash = BASE64.encode(digest(&SHA512, pass.as_bytes()).as_ref());
    let true_hash = std::env::var("ADMIN_PASSWORD_HASH")
        .expect("ADMIN_PASSWORD_HASH wasn't provided in environment");

    hash == true_hash
}

pub async fn adm_start(cx: &Cx, db: &DB) -> Res {
    match cx.update.from() {
        Some(user) => {
            let nickname = user.clone().username.expect("Must be user");

            if db.is_admin(user.id).await? {
                cx.answer(format!("@{} is already admin.", nickname))
                    .await?;
            } else {
                db.add_admin(user.id).await?;

                cx.answer(format!("@{} registered as bot admin.", nickname))
                    .await?;
            }
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
