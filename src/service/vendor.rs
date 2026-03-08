use sqlx::{Pool, Postgres, postgres::PgRow};

use crate::models::vendors::{Vacancy, Vendor};

pub async fn create_vendor(con: &Pool<Postgres>, vendor: Vendor) -> Result<(), sqlx::Error> {
   sqlx::query!(
        "
        INSERT INTO vendors (name, pw_hash, email, bw_rate, clrd_rate, location, availability, pub_id) 
         VALUES ($1, $2, $3, $4, $5, $6, $7::vacancy, $8);
        ",
        vendor.name, vendor.pw_hash, vendor.email, vendor.bw_rate, vendor.clrd_rate, vendor.location, vendor.availability as Vacancy, vendor.pub_id 
    )       
    .execute(con)
    .await?;

    Ok(())
}

pub async fn get_vendor(con: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
     let act = sqlx::query(
         
         r#" 
         SELECT * FROM vendors; 
         "#)

    .fetch_all(con)
    .await?;
        
     Ok(act) 
}


