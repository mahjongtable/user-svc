use async_trait::async_trait;
use sqlx::Error;

use super::entity::{CreateUser, UserEntity};

// pub trait User: Send {
//     fn uid(&self) -> Option<String>;
//     fn username(&self) -> Option<String>;
//     fn gender(&self) -> Option<i32>;
//     fn avatar_url(&self) -> Option<String>;
//     fn email(&self) -> String;
//     fn cellphone_number(&self) -> Option<String>;
//     fn password(&self) -> String;
// }

// #[derive(Debug)]
// pub struct User {
//     pub id: Option<u64>,
//     pub username: Option<String>,
//     pub gender: Option<i8>,
//     pub avatar_url: Option<String>,
//     pub email: String,
//     pub cellphone_number: Option<String>,
//     pub password: Option<String>,
//     pub created_at: Option<chrono::NaiveDateTime>,
//     pub updated_at: Option<chrono::NaiveDateTime>,
//     pub deleted_at: Option<chrono::NaiveDateTime>,
// }

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, uid: u64) -> Result<UserEntity, Error>;
    async fn create_user(&self, user: CreateUser) -> Result<u64, Error>;
    async fn delete_user(&self, uid: u64) -> Result<(), Error>;
}
