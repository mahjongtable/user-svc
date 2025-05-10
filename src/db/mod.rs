pub mod repository;

use crate::settings;
use repository::{User, UserRepository};
use sqlx::{pool::PoolOptions, Database, Error, MySql, Pool};
use async_trait::async_trait;

pub async fn connect<D: Database>(cfg: &settings::Database) -> Result<Pool<D>, Error> {
    let url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        &cfg.username,
        &cfg.password,
        &cfg.host,
        &cfg.port.unwrap_or(3306),
        &cfg.database
    );

    let pool = PoolOptions::<D>::new()
        .max_connections(cfg.max_connections.unwrap_or(5) as u32)
        .connect(&url)
        .await?;

    Ok(pool)
}

pub struct DbUserRepository {
    pub pool: Pool<MySql>,
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn get_user(&self, uid: i64) -> Result<User, Error> {
        let _sql = "SELECT `username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password` WHERE `id` = $1";
        todo!()
    }

    async fn create_user(&self, user: User) -> Result<i64, Error> {
        let username = user.username.unwrap_or("User".to_string());
        let gender = user.gender;
        let avatar_url = user.gender;
        let email = user.email;
        let cellphone_number = user.cellphone_number;
        let password = user.password;

        sqlx::query!(r#"
            INSERT INTO `users` (`username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password`) VALUES (?, ?, ?, ?, ?, ?)
        "#, username, gender, avatar_url, email, cellphone_number, password).execute(&self.pool).await?.last_insert_id();

        Ok(1)
    }

    async fn delete_user(&self, uid: i64) -> Result<(), Error> {
        todo!()
    }
}
