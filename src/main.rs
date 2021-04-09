use teloxide::prelude::*;

mod command;
mod database;
pub mod utils;

use command::*;
use database::*;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = new_database().await;
    let db = database.unwrap();

    run(db.clone()).await;
}

async fn run(db: Box<dyn Database>) {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, move |cx: Cx, command: Command| {
        answer(cx, command, db.clone())
    })
    .await
}
