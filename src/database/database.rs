use async_trait::async_trait;
use std::error::Error;

pub type Res<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait Database: Send + Sync {
    async fn add_admin(&self, id: i64) -> Res<()>;
    async fn is_admin(&self, id: i64) -> Res<bool>;
    async fn find_group(&self, member: i64) -> Res<Option<String>>;
    async fn create_group(&self, owner: i64) -> Res<String>;
    async fn add_group_member(&self, group: &String, member: i64) -> Res<()>;
    async fn get_group(&self, group: &String) -> Res<Option<()>>;
}
