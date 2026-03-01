use sqlx::{ Pool, Postgres, postgres::{PgPoolOptions} };

pub async fn connect() ->  Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://zacm:1234@localhost:5432/webprintingapi_db")
        .await?;

    Ok(pool)
}
