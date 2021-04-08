use teloxide::prelude::*;

mod command;
mod database;
pub mod utils;

use command::*;
use database::*;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;
use std::sync::Arc;
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = MongoDB::new().await;
    let db = Arc::new(database.unwrap());

    run(db.clone()).await;
}

async fn run(db: Arc<MongoDB>) {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, move |cx: Cx, command: Command| {
        answer(cx, command, db.clone())
    })
    .await
}
