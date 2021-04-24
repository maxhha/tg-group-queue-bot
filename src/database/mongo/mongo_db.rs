use crate::database::{Database, Res};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::error::Error;
use tokio_compat_02::FutureExt;

/// ## Database structure
/// ```graphql
///
/// scalar TelegramID # i64 rust type
///
/// type Database {
///     groups: [Group!]!
///     queues: [Queue!]!
///     globalBans: [TelegramUser!]!
///     admins: [TelegramUser!]!
/// }
///
/// type Group {
///     _id: ID!
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
///     _id: ID!
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
/// # Use only on database level for collections
/// type TelegramUser {
///     _id: TelegramID!
/// }
///
/// ```
pub struct MongoDB {
    database: mongodb::Database,
}

impl MongoDB {
    pub async fn new() -> Result<MongoDB, Box<dyn Error>> {
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

#[async_trait]
impl Database for MongoDB {
    async fn add_admin(&self, id: i64) -> Res<()> {
        self.database
            .collection("admins")
            .insert_one(doc! { "_id": id }, None)
            .await?;
        Ok(())
    }

    async fn is_admin(&self, id: i64) -> Res<bool> {
        let user: Option<mongodb::bson::Document> = self
            .database
            .collection("admins")
            .find_one(doc! { "_id": id }, None)
            .await?;

        Ok(user.is_some())
    }

    async fn find_group(&self, member: i64) -> Res<Option<String>> {
        Ok(None)
    }

    async fn create_group(&self, owner: i64) -> Res<String> {
        Ok("test_group".into())
    }

    async fn add_group_member(&self, group: &String, member: i64) -> Res<()> {
        Ok(())
    }

    async fn get_group(&self, group: &String) -> Res<Option<()>> {
        Ok(Some(()))
    }
}
