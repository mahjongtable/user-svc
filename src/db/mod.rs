use sqlx::{Error, Pool, Postgres, postgres::PgPoolOptions};

pub async fn connect() -> Result<Pool<Postgres>, Error> {
    let host = "127.0.0.1";
    let username = "root";
    let password = "root";
    let database = "user";

    let url = format!(
        "postgresql://{}:{}@{}/{}",
        username, password, host, database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}
