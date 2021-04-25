#[macro_use]
extern crate enum_display_derive;
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

    let database = Arc::new(
        new_database()
            .await
            .expect("Failed to create database client"),
    );

    run(database.clone()).await;
}

async fn run(db: Arc<Box<dyn Database>>) {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, move |cx: Cx, command: Command| {
        answer(cx, command, db.clone())
    })
    .await
}
