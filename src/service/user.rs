use sqlx::{Pool, Postgres, postgres::PgRow};

use crate::models::{users::User};

pub async fn create_user(con: &Pool<Postgres>, user: User) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO users (name, pw_hash, email, pub_id) VALUES ($1, $2, $3, $4)",
        user.name,
        user.pw_hash,
        user.email,
        user.pub_id
        ) 
    .execute(con)
    .await?;

    Ok(())
}

pub async fn get_users(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
     let act = sqlx::query(
         
         r#" 
         SELECT * FROM users 
         "#)

    .fetch_all(con)
    .await?;
        
     Ok(act) 
}

