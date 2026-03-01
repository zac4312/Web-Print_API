use sqlx::{Pool, Postgres, postgres::PgRow};

pub async fn get_partner(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
     let act = sqlx::query(
         
         r#" 
         SELECT * FROM partners; 
         "#)

    .fetch_all(con)
    .await?;
        
     Ok(act) 
}

pub async fn create_partner(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
    let act = sqlx::query(
    
        r#"
        INSERT INTO partners (name, pw_hash, bw_rate, clrd_rate, location, availability) 
        VALUES ('TEST', 'TEST', 32.50, 32.55, '(139.6917, 35.6895)', 'busy');
        "#)
    
    .fetch_all(con)
    .await?;

    Ok(act)
}



