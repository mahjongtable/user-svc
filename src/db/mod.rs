pub mod dto;
pub mod entity;
pub mod repository;

use crate::settings;
use async_trait::async_trait;
use entity::{CreateUser, UpdateUser, UserEntity};
use repository::{RepositoryError, UserRepository};
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

    async fn create_user(&self, user: CreateUser) -> Result<u64, Error> {
        let username = user.username;
        let gender = user.gender;
        let avatar_url = user.avatar_url;
        let email = user.email;
        let cellphone_number = user.cellphone_number;
        let password = user.password;

        let new_id = sqlx::query!(
            r#"
            INSERT INTO `users` (`username`, `gender`, `avatar_url`, `email`, `cellphone_number`, `password`) VALUES (?, ?, ?, ?, ?, ?)
            "#,
            username,
            gender,
            avatar_url,
            email,
            cellphone_number,
            password,
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(new_id)
    }

    async fn delete_user(&self, uid: u64) -> Result<(), Error> {
        let sql = "UPDATE `users` SET `deleted_at` = NOW() WHERE `id` = ? AND `deleted_at` IS NULL";

        sqlx::query(sql).bind(uid).execute(&self.pool).await?;

        // todo: Need to check whether the user already exist
        // and need to check whether the user has been deleted.

        Ok(())
    }

    async fn update_user(&self, uid: u64, user: UpdateUser) -> Result<(), RepositoryError> {
        let sql = "SELECT 1 FROM `users` WHERE `id` = ?";

        if sqlx::query(sql).bind(uid).fetch_optional(&self.pool).await?.is_none() {
            return Err(RepositoryError::RecordNotFound);
        }

        let sql = r#"
        UPDATE IGNORE
            `users`
        SET
            `username` = ?,
            `gender` = ?,
            `avatar_url` = ?,
            `email` = ?,
            `cellphone_number` = ?
        WHERE `id` = ?
        "#;

        sqlx::query(sql)
            .bind(user.username)
            .bind(user.gender)
            .bind(user.avatar_url)
            .bind(user.email)
            .bind(user.cellphone_number)
            .bind(uid)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
