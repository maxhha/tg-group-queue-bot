mod database;
mod mongo;

pub use database::*;
use mongo::MongoDB;

pub async fn new_database() -> Result<Box<dyn Database>, Box<dyn std::error::Error>> {
    Ok(Box::new(MongoDB::new().await?))
}
