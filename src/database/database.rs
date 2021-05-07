use async_trait::async_trait;
use std::error::Error;
use teloxide::types::Document;

pub type Res<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct Group {
    pub id: String,
    pub owner: i64,
    pub members: Vec<Member>,
    pub queues: Vec<String>,
    pub bans: Vec<i64>,
}

pub struct Member {
    pub id: i64,
    pub name: Option<String>,
}

#[async_trait]
pub trait Database: Send + Sync {
    async fn add_admin(&self, id: i64) -> Res<()>;
    async fn is_admin(&self, id: i64) -> Res<bool>;
    async fn find_group(&self, member: i64) -> Res<Option<String>>;
    async fn create_group(&self, owner: i64) -> Res<String>;
    async fn add_group_member(&self, group: &String, member: i64) -> Res<()>;
    async fn get_group(&self, group: &String) -> Res<Option<Group>>;

    async fn set_username(&self, member: i64, username: &String) -> Res<()>;

    async fn find_queue(&self, group: &String, subject: &String) -> Res<Option<String>>;
    async fn pop_first_queue_pos(&self, queueid: &String) -> Res<String>;

    async fn add_subject(&self, owner: i64, subject: &String) -> Res<(String)>;
    async fn find_subject(&self, subject: &String) -> Res<Option<String>>;
    async fn rm_subject(&self, owner: i64, subject: &String) -> Res<(String)>;
}
