use sqlx::{Pool, Postgres, Result};

use crate::models::vendors::Vendor;

pub async fn choose_vendor(con: &Pool<Postgres>, vendor: Vendor) -> Result<Vendor, sqlx::Error>{
    let state = sqlx::query_file!("sql_queries/choose_vendor.sql",
            vendor.id
        )
        .execute(con)
        .await?;

    Ok(state)
}
