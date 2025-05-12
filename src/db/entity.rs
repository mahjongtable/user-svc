use chrono::{DateTime, Local, Utc};
use prost_types::Timestamp;

use crate::pb::user::{CreateUserRequest, GetUserResponse};

#[derive(sqlx::FromRow, Debug)]
pub struct UserEntity {
    pub id: u64,
    pub username: Option<String>,
    pub gender: Option<i8>,
    pub avatar_url: Option<String>,
    pub email: String,
    pub cellphone_number: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl UserEntity {
    pub fn created_at_utc(&self) -> DateTime<Utc> {
        todo!()
    }
}

impl From<UserEntity> for GetUserResponse {
    fn from(value: UserEntity) -> Self {
        // let created_at = value.created_at.unwrap_or_default().and_utc().timestamp();

        Self {
            id: value.id,
            username: value.username.unwrap_or(format!("User {}", value.id)),
            gender: value.gender.unwrap_or(0) as i32,
            avatar_url: value.avatar_url,
            email: value.email,
            cellphone_number: value.cellphone_number,
            created_at: None, // todo: need to convert to the timestamp type of proto3.
            updated_at: None, // todo: need to convert to the timestamp type of proto3.
        }
    }
}

impl From<CreateUserRequest> for UserEntity {
    fn from(value: CreateUserRequest) -> Self {
        let now = Local::now().naive_local();

        Self {
            id: 0,
            username: value.username,
            gender: value.gender.map(|g| g as i8),
            avatar_url: value.avatar_url,
            email: value.email,
            cellphone_number: value.cellphone_number,
            password: value.password,
            created_at: Some(now),
            updated_at: Some(now),
            deleted_at: None,
        }
    }
}
