use mongodb::{options::ClientOptions, Client};
use std::env;
use std::future::Future;
use tokio_compat_02::FutureExt;

// trait Database {
//     fn new() -> Box<dyn Future<Output=Result<Self, Box<dyn std::error::Error>>>>;
// }
//
// struct MongoDB(mongodb::Database);
//
// impl Database for MongoDB {
//     async fn new() -> Result<Self, Box<dyn std::error::Error>> {
//         let mut options = ClientOptions::parse(env::var("MONGO_DSL").unwrap().as_str())
//             .compat()
//             .await
//             .unwrap();
//
//         options.app_name = Some(env::var("BOT_NAME").unwrap());
//
//         let client = Client::with_options(options).unwrap();
//
//         Ok(MongoDB(client.database(env::var("DB_NAME").unwrap().as_str())))
//     }
// }
