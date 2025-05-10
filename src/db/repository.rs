use sqlx::Error;
use async_trait::async_trait;

// pub trait User: Send {
//     fn uid(&self) -> Option<String>;
//     fn username(&self) -> Option<String>;
//     fn gender(&self) -> Option<i32>;
//     fn avatar_url(&self) -> Option<String>;
//     fn email(&self) -> String;
//     fn cellphone_number(&self) -> Option<String>;
//     fn password(&self) -> String;
// }

pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub gender: Option<i32>,
    pub avatar_url: Option<String>,
    pub email: String,
    pub cellphone_number: Option<String>,
    pub password: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, uid: i64) -> Result<User, Error>;
    async fn create_user(&self, user: User) -> Result<i64, Error>;
    async fn delete_user(&self, uid: i64) -> Result<(), Error>;
}
