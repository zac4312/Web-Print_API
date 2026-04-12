use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{dto::user::{GetUser}, models::users::User};

pub async fn login_user(con: &Pool<Postgres>, pw: String) -> Result<String, sqlx::Error> {
    let attempt = sqlx::query!(
        "
        select pub_id from users
        where pw_hash = $1;
        ", pw)
        .fetch_one(con)
        .await?;

    Ok(attempt.pub_id)
}

pub async fn create_user(con: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO users (name, pw_hash, email, pub_id) VALUES ($1, $2, $3, $4)",
            user.name,
            user.pw_hash,
            user.email,
            user.pub_id )
        .execute(con)
        .await?;


    Ok(())
}

pub async fn get_users(con: &Pool<Postgres>) -> Result<Vec<GetUser>, sqlx::Error> {
     let act = sqlx::query_as!(GetUser, "SELECT pub_id, name, email FROM users")
        .fetch_all(con)
        .await?;

     Ok(act) 
}

pub async fn map_user(con: &Pool<Postgres>, pub_id: &String) -> Result<Uuid, sqlx::Error> {
    let db_id = sqlx::query_file!("sql_queries/map_users.sql", pub_id)
        .fetch_one(con)
        .await?;
    Ok(db_id.user_id)
}

 

