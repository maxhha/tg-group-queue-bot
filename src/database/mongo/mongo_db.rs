use crate::database::Database;
use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio_compat_02::FutureExt;
// use async_trait::async_trait;

pub struct MongoDB {
    database: mongodb::Database,
}

// #[async_trait]
impl MongoDB {
    pub async fn new() -> Result<MongoDB, Box<dyn std::error::Error>> {
        let mut options = ClientOptions::parse(env::var("MONGO_DSL").unwrap().as_str())
            .compat()
            .await
            .unwrap();

        options.app_name = Some(env::var("BOT_NAME").unwrap());

        let client = Client::with_options(options).unwrap();

        Ok(MongoDB {
            database: client.database(env::var("DB_NAME").unwrap().as_str()),
        })
    }
}

impl Database for MongoDB {}
