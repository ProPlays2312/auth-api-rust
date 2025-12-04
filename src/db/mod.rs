// src/db/mod.rs
use async_trait::async_trait;
use crate::models::user::{User, CreateUserSchema};
use std::sync::Arc;

pub mod sql;
pub mod surreal;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create_user(&self, user: CreateUserSchema) -> Result<User, anyhow::Error>;
    // Updated to match the implementation
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
}

pub type Db = Arc<dyn UserRepo>;