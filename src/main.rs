use teloxide::prelude::*;

mod command;
pub mod utils;

use command::*;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;
use std::sync::Arc;
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut options = ClientOptions::parse(env::var("MONGO_DSL").unwrap().as_str())
        .compat()
        .await
        .unwrap();

    options.app_name = Some("tg-queue-bot".to_string());

    let client = Client::with_options(options).unwrap();

    let db = Arc::new(client.database("develop"));

    run(db.clone()).await;
}

async fn run(db: Arc<Database>) {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, move |cx: Cx, command: Command| {
        answer(cx, command, db.clone())
    })
    .await
}
