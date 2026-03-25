use sqlx::{Pool, Postgres, Result};

use crate::{ err::ServiceErr, models::{vendors::Vacancy} };

pub async fn choose_vendor(con: &Pool<Postgres>, ui: &String) -> Result<String, ServiceErr>{
    let query = sqlx::query_file!("sql_queries/choose_vendor.sql", ui)     
        .fetch_optional(con)
        .await?;

    let vendor = query.ok_or(ServiceErr::VendorNotFound)?;
    if vendor.availability == Vacancy::Closed {return Err(ServiceErr::VendorUnavailable)}

    Ok(vendor.pub_id)
}

pub async fn attach_file(con &Pool<Postgres>, ui: &String) -> Result<String, sqlx::Error> {
     let ;

    Ok
}

