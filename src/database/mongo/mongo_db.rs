use crate::database::Database;
use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio_compat_02::FutureExt;
// use async_trait::async_trait;

/// ## Database structure
/// ```graphql
///
/// scalar TelegramID
///
/// type Database {
///     groups: [Group!]!
///     queues: [Queue!]!
///     globalBans: [TelegramID!]!
///     admins: [TelegramID!]!
/// }
///
/// type Group {
///     id: ID!
///     owner: TelegramID!
///     members: [Member!]!
///     queues: [ID!]!
///     bans: [TelegramID!]!
/// }
///
/// type Member {
///     id: TelegramID!
///     name: String
/// }
///
/// type Queue {
///     id: ID!
///     groupId: ID!
///     name: String!
///     records: [QueueRecord!]!
/// }
///
/// type QueueRecord {
///     user: TelegramID!
///     message: String!
/// }
///
/// ```
pub struct MongoDB {
    database: mongodb::Database,
}

impl MongoDB {
    pub async fn new() -> Result<MongoDB, Box<dyn std::error::Error>> {
        let mut options = ClientOptions::parse(env::var("MONGO_DSL").unwrap().as_str())
            .compat()
            .await
            .unwrap();

        options.app_name = Some(env::var("BOT_NAME").unwrap());

        let client = Client::with_options(options).unwrap();

        Ok(MongoDB {
            database: client.database(env::var("DATABASE_NAME").unwrap().as_str()),
        })
    }
}

impl Database for MongoDB {}
