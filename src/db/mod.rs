pub mod repository;
pub mod entity;

use crate::settings;
use async_trait::async_trait;
use entity::UserEntity;
use repository::UserRepository;
use sqlx::{Database, Error, MySql, Pool, pool::PoolOptions};

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
    async fn get_user(&self, uid: u64) -> Result<UserEntity, Error> {
        let user = sqlx::query_as!(UserEntity, r#"SELECT `id`, `username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password`, `created_at`, `updated_at`, `deleted_at` FROM `users` WHERE `id` = ?"#, uid).fetch_one(&self.pool).await?;
        Ok(user)
    }

    async fn create_user(&self, user: UserEntity) -> Result<u64, Error> {
        let username = user.username;
        let gender = user.gender;
        let avatar_url = user.avatar_url;
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
