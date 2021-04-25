use crate::database::{Database, Group, Member, Res};
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::error::Error;
use std::fmt::Display;
use tokio_compat_02::FutureExt;

#[derive(Debug, Display)]
pub enum MongoDBError {
    FailCreateObjectIdError,
    InvalidDataTypeError(String),
}

impl Error for MongoDBError {}

use MongoDBError::*;

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
        let group = self
            .database
            .collection("groups")
            .insert_one(
                doc! {
                    "owner": owner,
                    "members": [],
                    "queues": [],
                    "bans": []
                },
                None,
            )
            .await?
            .inserted_id;

        match group {
            bson::Bson::ObjectId(id) => Ok(id.to_hex()),
            _ => Err(Box::new(FailCreateObjectIdError)),
        }
    }

    async fn add_group_member(&self, group: &String, member: i64) -> Res<()> {
        Ok(())
    }

    async fn get_group(&self, group: &String) -> Res<Option<Group>> {
        let group: Option<mongodb::bson::Document> = self
            .database
            .collection("groups")
            .find_one(
                doc! { "_id": bson::oid::ObjectId::with_string(group)? },
                None,
            )
            .await?;

        if let Some(group) = group {
            let id = group
                .get_object_id("_id")
                .map_err(|_| InvalidDataTypeError("group._id".into()))?
                .to_hex();

            let owner = group
                .get_i64("owner")
                .map_err(|_| InvalidDataTypeError("group.owner".into()))?;

            let members = group
                .get_array("members")
                .map_err(|_| InvalidDataTypeError("group.members".into()))?
                .into_iter()
                .enumerate()
                .map::<Result<_, MongoDBError>, _>(|(i, x)| {
                    let x = x
                        .as_document()
                        .ok_or(InvalidDataTypeError(format!("group.members[{}]", i)))?;

                    Ok(Member {
                        id: x.get_i64("id").map_err(|_| {
                            InvalidDataTypeError(format!("group.members[{}].id", i))
                        })?,
                        name: x.get_str("name").ok().map(|x| x.into()),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let queues = group
                .get_array("queues")
                .map_err(|_| InvalidDataTypeError("group.queues".into()))?
                .into_iter()
                .enumerate()
                .map::<Result<_, MongoDBError>, _>(|(i, x)| {
                    Ok(x.as_str()
                        .ok_or(InvalidDataTypeError(format!("queues[{}]", i)))?
                        .into())
                })
                .collect::<Result<Vec<_>, _>>()?;

            let bans = group
                .get_array("bans")
                .map_err(|_| InvalidDataTypeError("group.bans".into()))?
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    x.as_i64()
                        .ok_or(InvalidDataTypeError(format!("group.bans[{}]", i)))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Some(Group {
                id,
                owner,
                members,
                queues,
                bans,
            }))
        } else {
            Ok(None)
        }
    }
}
