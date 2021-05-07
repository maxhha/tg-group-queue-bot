use crate::database::{Database, Group, Member, Res};
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::Display;
use tokio_compat_02::FutureExt;

#[derive(Debug, Display)]
pub enum MongoDBError {
    FailCreateObjectIdError,
}

impl Error for MongoDBError {}

use MongoDBError::*;

#[derive(Serialize, Deserialize)]
struct MongoGroup {
    _id: ObjectId,
    owner: i64,
    members: Vec<MongoMember>,
    queues: Vec<String>,
    bans: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
struct MongoMember {
    id: i64,
    name: Option<String>,
}

impl Into<Member> for MongoMember {
    fn into(self) -> Member {
        Member {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<Group> for MongoGroup {
    fn into(self) -> Group {
        Group {
            id: self._id.to_hex(),
            owner: self.owner,
            members: self.members.into_iter().map(|x| x.into()).collect(),
            queues: self.queues,
            bans: self.bans,
        }
    }
}

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
        let user: Option<bson::Document> = self
            .database
            .collection("admins")
            .find_one(doc! { "_id": id }, None)
            .await?;

        Ok(user.is_some())
    }

    async fn find_group(&self, member: i64) -> Res<Option<String>> {
        let group: Option<bson::Document> = self
            .database
            .collection("groups")
            .find_one(
                doc! {
                    "members.id": member
                },
                None,
            )
            .await?;

        if let Some(group) = group {
            Ok(Some(group.get_object_id("_id")?.to_hex()))
        } else {
            Ok(None)
        }
    }

    async fn create_group(&self, owner: i64) -> Res<String> {
        let group = self
            .database
            .collection("groups")
            .insert_one(
                doc! {
                    "owner": owner,
                    "members": [{
                        "id": owner,
                    }],
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
        self.database
            .collection::<bson::Document>("groups")
            .update_one(
                doc! {
                    "_id": ObjectId::with_string(group)?,
                    "members.id": { "$ne": member }
                },
                doc! {
                    "$push": {
                        "members": { "id": member }
                    }
                },
                None,
            )
            .await?;

        Ok(())
    }

    async fn get_group(&self, group: &String) -> Res<Option<Group>> {
        let group: Option<mongodb::bson::Document> = self
            .database
            .collection("groups")
            .find_one(doc! { "_id": ObjectId::with_string(group)? }, None)
            .await?;

        if let Some(group) = group {
            Ok(Some(bson::from_document::<MongoGroup>(group)?.into()))
        } else {
            Ok(None)
        }
    }

    async fn set_username(&self, member: i64, username: &String) -> Res<()> {
        let group = self.find_group(member).await?;

        if let Some(group) = group {
            self.database
                .collection::<bson::Document>("groups")
                .update_one(
                    doc! {
                        "_id": ObjectId::with_string(&group)?,
                        "members.id": member
                    },
                    doc! {
                        "$set": {
                            "members.$": { "id": member, "name": username }
                        }
                    },
                    None,
                )
                .await?;
        }

        Ok(())
    }

    async fn find_queue(&self, group: &String, subject: &String) -> Res<Option<String>> {
        let queue: Option<mongodb::bson::Document> = self
            .database
            .collection::<bson::Document>("queues")
            .find_one(
                doc! {
                    "groupId": (group.clone()),
                    "name": subject,
                },
                None,
            )
            .await?;

        if let Some(queue) = queue {
            Ok(Some(queue.get_object_id("_id")?.to_hex()))
        } else {
            Ok(None)
        }
    }

    async fn pop_first_queue_pos(&self, queueid: &String) -> Res<String> {
        self.database
            .collection::<bson::Document>("queues")
            .update_one(
                doc! {
                    "_id": ObjectId::with_string(&queueid)?,
                },
                doc! {
                    "$pop": {
                        "records": -1
                    }
                },
                None,
            )
            .await?;

        Ok("".to_string())
    }

    async fn add_subject(&self, owner: i64, subject: &String) -> Res<(String)> {
        let group = self.find_group(owner).await?.unwrap();

        let queue = self
            .database
            .collection::<bson::Document>("queues")
            .insert_one(
                doc! {
                    "groupId": (group.clone()),
                    "name": subject,
                    "records" : []
                },
                None,
            )
            .await?
            .inserted_id;

        self.database
            .collection::<bson::Document>("groups")
            .update_one(
                doc! {
                    "_id": ObjectId::with_string(&group)?,
                },
                doc! {
                    "$push": {
                        "queues": { "id": (queue.clone()) }
                    }
                },
                None,
            )
            .await?;

        Ok(queue.to_string())
    }

    async fn find_subject(&self, subject: &String) -> Res<Option<String>> {
        let subj: Option<bson::Document> = self
            .database
            .collection("queues")
            .find_one(
                doc! {
                    "name": subject
                },
                None,
            )
            .await?;

        if let Some(group) = subj {
            Ok(Some(group.get_object_id("_id")?.to_hex()))
        } else {
            Ok(None)
        }
    }

    async fn rm_subject(&self, owner: i64, subject: &String) -> Res<(String)> {
        let group = self.find_group(owner).await?.unwrap();

        let queue = self.find_subject(subject).await?;

        if let Some(queue) = queue {
            let id = ObjectId::with_string(&queue)?;
            self.database
                .collection::<bson::Document>("groups")
                .update_one(
                    doc! {
                        "_id": ObjectId::with_string(&group)?,
                    },
                    doc! {
                        "$pull": {
                            "queues": { "id": (id.clone()) }
                        }
                    },
                    None,
                )
                .await?;

            self.database
                .collection::<bson::Document>("queues")
                .delete_one(
                    doc! {
                        "_id": id,
                    },
                    None,
                )
                .await?;
        }

        Ok(("".to_string()))
    }
}
