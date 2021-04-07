use teloxide::prelude::*;

mod command;
pub mod utils;

use command::command::answer;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // // Parse a connection string into an options struct.
    // let mongodb_uri = String::from("mongodb://cluster-shard-00-00.bwags.mongodb.net:27017, \
    //     mongodb://cluster-shard-00-01.bwags.mongodb.net:27017, \
    //     mongodb://cluster-shard-00-02.bwags.mongodb.net:27017/");

    // let opts = ClientOptions::

    let mut client_options = ClientOptions::parse(
        "mongodb+srv://bot:<password>@cluster.bwags.mongodb.net/develop?retryWrites=true&w=majority",
    )
    .compat()
    .await
    .unwrap();

    // Manually set an option.
    client_options.app_name = Some("tg-queue-bot".to_string());

    println!("Create client");

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();
    println!("query database");
    let db = client.database("develop");
    println!("query collections");

    for collection_name in db.list_collection_names(None).compat().await.unwrap() {
        println!(" - {}", collection_name);
    }

    // run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting group-queue ...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "group-queue".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
