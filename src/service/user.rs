use sqlx::{Pool, Postgres, postgres::PgRow};

pub async fn get_users(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
     let act = sqlx::query(
         
         r#" 
         SELECT * FROM users 
         "#)

    .fetch_all(con)
    .await?;
        
     Ok(act) 
}

pub async fn create_user(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
    let act = sqlx::query(
    
        r#"
        INSERT INTO users (name, pw_hash) 
        VALUES ('TEST', 'TEST');
        "#)
    
    .fetch_all(con)
    .await?;

    Ok(act)
}
