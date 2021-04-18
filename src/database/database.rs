use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Database: Send + Sync {
    async fn add_admin(&self, id: i64) -> Result<(), Box<dyn Error + Send + Sync>>;
}
