use chrono::{DateTime, Utc};

use crate::pb::user::{CreateUserRequest, GetUserResponse};

#[derive(sqlx::FromRow, Debug)]
pub struct UserEntity {
    pub id: u64,
    // It's not null in the database, so we will provide default value.
    pub username: Option<String>,
    // It's not null in the database, so we will provide default value.
    pub gender: Option<i8>,
    // The avatar url will be confirmed by client.
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

// The some fields are from the "entity.rs/UserEntity".
pub struct CreateUser {
    // pub id: u64,
    // It's not null in the database, so we will provide default value.
    pub username: String,
    // It's not null in the database, so we will provide default value.
    pub gender: i32,
    // It will be confirmed by client.
    pub avatar_url: Option<String>,
    pub email: String,
    pub cellphone_number: Option<String>,
    pub password: Option<String>,
    // Don't need the created_at/updated_at/deleted_at,
    // because the columns will be initialed, when the record was created.
    // created_at..
    // updated_at..
    // deleted_at..
}

impl From<CreateUserRequest> for CreateUser {
    fn from(value: CreateUserRequest) -> Self {
        Self {
            username: value.username.unwrap_or(String::from("新用户")),
            gender: value.gender.unwrap_or(1),
            avatar_url: value.avatar_url,
            email: value.email,
            cellphone_number: value.cellphone_number,
            password: value.password,
        }
    }
}

impl From<UserEntity> for GetUserResponse {
    fn from(value: UserEntity) -> Self {
        // let created_at = value.created_at.unwrap_or_default().and_utc().timestamp();

        Self {
            id: value.id,
            username: value.username,
            gender: value.gender.unwrap_or(0) as i32,
            avatar_url: value.avatar_url,
            email: value.email,
            cellphone_number: value.cellphone_number,
            created_at: None, // todo: need to convert to the timestamp type of proto3.
            updated_at: None, // todo: need to convert to the timestamp type of proto3.
        }
    }
}

fn hash_password(password: &str) -> String {
    todo!()
}