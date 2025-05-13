
// The fields are from the "entity.rs/UserEntity".
#[deprecated]
pub struct CreateUserDto {
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
