use crate::models::user::{CreateUserSchema, User};
// src/db/mod.rs
use async_trait::async_trait;
use std::sync::Arc;

pub mod connection;
pub mod sql;

pub use connection::{connect, test_database_connection};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create_user(&self, user: CreateUserSchema) -> Result<User, anyhow::Error>;
    // Updated to match the implementation
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
}

pub type Db = Arc<dyn UserRepo>;