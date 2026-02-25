mod models;
use sqlx::{PgPool, postgres::PgRow};
use models::md_user::User;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!"); 
    let test = PgPool::connect("postgresql://zacm:1234@localhost:5432/webprintingapi_db").await?;
    let res = get(&test).await?; 
    println!("{:?}", res);
    Ok(()) 
}

async fn get(con: &PgPool) -> Result<Vec<PgRow>, sqlx::Error> {
     let act = sqlx::query(r#" SELECT * FROM users "#)
         .fetch_all(con)
         .await?;
        
     Ok(act)
}

