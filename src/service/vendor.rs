use sqlx::{Pool, Postgres, postgres::PgRow};

use crate::{dto::vendor::GetVendors, err::{TransactionErr, VendorErr}, models::vendors::{Vendor}};

pub async fn create_vendor(con: &Pool<Postgres>, vendor: Vendor) -> Result<(), sqlx::Error> {
   sqlx::query(
        "
        INSERT INTO vendors (name, pw_hash, email, bw_rate, clrd_rate, lat, long, availability, pub_id, brand, gcash) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::vacancy, $10, $11);
        ")

       .bind(vendor.name) .bind(vendor.pw_hash) .bind(vendor.email) .bind(vendor.bw_rate) .bind(vendor.clrd_rate) .bind(vendor.lat) .bind(vendor.long) .bind(vendor.availability) .bind(vendor.pub_id)
    .execute(con)
    .await?; 

    Ok(())
}

pub async fn get_vendor(con: &Pool<Postgres>) -> Result<Vec<GetVendors>, sqlx::Error> {
     let act = sqlx::query_as::<_, GetVendors>("SELECT pub_id, email, bw_rate, clrd_rate, lat, long, availability, brand FROM vendors")


    .fetch_all(con)
    .await?;
        
     Ok(act) 
}

pub async fn list_orders(con: &Pool<Postgres>, ui: &String) -> Result<Vec<PgRow>, TransactionErr> {
    let orders = sqlx::query
        
        ("
            SELECT o.color, o.copies, o.print_size, o.pub_id, o.status, o.total, u.name, f.file_path
            FROM orders o
            LEFT JOIN vendors v
            ON o.for_vendor = v.vendor_id
            LEFT JOIN users u
            ON u.user_id = o.for_user
            LEFT JOIN files f
            ON f.file_id = o.file_id
            WHERE v.pub_id = '$1';        
        ")
        .bind(ui)
        .fetch_all(con)
        .await?;

    Ok(orders)
}

pub async fn accept_order(con: &Pool<Postgres>, ui: &String) -> Result<(), VendorErr> {
    let accept_query = sqlx::query_file!("sql_queries/accept_order.sql", ui)
        .execute(con)
        .await?;
    
    if accept_query.rows_affected() == 0 {
        return Err(VendorErr::OrderNotFoud);
    }
     
    Ok(())
}

pub async fn reject_order(con: &Pool<Postgres>, ui: &String) -> Result<(), VendorErr> {
    let reject_query = sqlx::query_file!("sql_queries/reject_order.sql", ui)
      .execute(con)
      .await?;

    if reject_query.rows_affected() == 0 {
        return Err(VendorErr::OrderNotFoud);
    }

    Ok(())
}
