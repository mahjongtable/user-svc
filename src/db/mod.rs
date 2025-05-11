pub mod repository;

use std::time;

use crate::settings;
use async_trait::async_trait;
use repository::{User, UserRepository};
use sqlx::{Database, Error, Executor, MySql, Pool, pool::PoolOptions};

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
    async fn get_user(&self, uid: u64) -> Result<User, Error> {
        let _sql = "SELECT `username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password` WHERE `id` = $1";
        todo!()
    }

    async fn create_user(&self, user: User) -> Result<u64, Error> {
        let username = user.username.unwrap_or("User".to_string());
        let gender = user.gender;
        let avatar_url = user.gender;
        let email = user.email;
        let cellphone_number = user.cellphone_number;
        let password = user.password;

        let now_datetime = chrono::DateTime::<chrono::Utc>::from(chrono::Utc::now());

        let new_id = sqlx::query!(
            r#"
            INSERT INTO `users` (`username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password`, `created_at`, `updated_at`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            username,
            gender,
            avatar_url,
            email,
            cellphone_number,
            password,
            now_datetime,
            now_datetime
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(new_id)
    }

    async fn delete_user(&self, uid: u64) -> Result<(), Error> {
        let _r = sqlx::query("DELETE FROM `users` WHERE `id` = ?")
            .bind(uid)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
