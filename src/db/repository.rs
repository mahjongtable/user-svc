use async_trait::async_trait;
use sqlx::Error;

use super::entity::{CreateUser, UpdateUser, UserEntity};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, uid: u64) -> Result<UserEntity, Error>;
    async fn create_user(&self, user: CreateUser) -> Result<u64, Error>;
    async fn delete_user(&self, uid: u64) -> Result<(), Error>;
    async fn update_user(&self, uid: u64, user: UpdateUser) -> Result<(), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("The record does not found.")]
    RecordNotFound,

    #[error("The database error: {0}")]
    Database(Box<dyn std::error::Error + Send + Sync>),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => RepositoryError::RecordNotFound,
            other_err => RepositoryError::Database(Box::new(other_err)),
        }
    }
}