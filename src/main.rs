use teloxide::prelude::*;

mod command;
pub mod utils;

use command::command::answer;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
