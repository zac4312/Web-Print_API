use sqlx::{Pool, Postgres, postgres::PgRow};

use crate::{dto::vendor::{GetVendors, HandlingOrders, OwnedOrders, VendorHome}, err::{TransactionErr, VendorErr}, models::vendors::{Vacancy, Vendor}};

pub async fn add_gcash(con: &Pool<Postgres>, file_path: String, pub_id: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    UPDATE vendors 
    set gcash = $1
    WHERE pub_id = $2;
        ", file_path, pub_id)
        .execute(con)
        .await?;

    Ok(())
}

pub async fn change_availability(con: &Pool<Postgres>, pub_id: String, new_state: Vacancy) -> Result<(), sqlx::Error> {
    sqlx::query(
        " 
    UPDATE vendors 
    set availability = $1
    WHERE pub_id = $2
        ")
        .bind(new_state) .bind(pub_id)
        .execute(con)
        .await?;
    Ok(())
}

pub async fn get_vendor_home(con: &Pool<Postgres>, pub_id: String) -> Result<Vec<VendorHome>, sqlx::Error> {
    let location = sqlx::query_as!(VendorHome,
        
        "
        SELECT lat, long from vendors
        where pub_id = $1
        ", pub_id)

        .fetch_all(con)
        .await?;
    Ok(location)
}

pub async fn create_vendor(con: &Pool<Postgres>, vendor: Vendor) -> Result<(), sqlx::Error> {
   sqlx::query(
        "
        INSERT INTO vendors (name, pw_hash, email, bw_rate, clrd_rate, lat, long, availability, pub_id, brand, gcash) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::vacancy, $9, $10, $11);
        ")

       .bind(vendor.name) .bind(vendor.pw_hash) .bind(vendor.email) .bind(vendor.bw_rate) .bind(vendor.clrd_rate) .bind(vendor.lat) .bind(vendor.long) .bind(vendor.availability) .bind(vendor.pub_id) .bind(vendor.brand) .bind(vendor.gcash)
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

pub async fn list_orders(con: &Pool<Postgres>, ui: &String) -> Result<Vec<OwnedOrders>, sqlx::Error> {
    let orders = sqlx::query_as::<_, OwnedOrders>(
        "
            SELECT o.color, o.copies, o.print_size, o.pub_id, o.status, o.total, u.name, f.file_path
            FROM orders o
            LEFT JOIN vendors v
            ON o.for_vendor = v.vendor_id
            LEFT JOIN users u
            ON u.user_id = o.for_user
            LEFT JOIN files f
            ON f.file_id = o.file_id
            WHERE 
            v.pub_id = $1
            and
            o.status = 'pending';        
 
        ").bind(ui)
        .fetch_all(con)
        .await?;

    Ok(orders)
}

pub async fn list_handling_orders(con: &Pool<Postgres>, ui: &String) -> Result<Vec<HandlingOrders>, sqlx::Error> {
   let orders = sqlx::query_as::<_, HandlingOrders>(
       "
        SELECT o.color, o.copies, o.print_size, o.pub_id, o.status, o.total, u.name, f.file_path, o.reciept

        FROM orders o
        LEFT JOIN vendors v
        ON o.for_vendor = v.vendor_id
        LEFT JOIN users u
        ON u.user_id = o.for_user
        LEFT JOIN files f
        ON f.file_id = o.file_id
        WHERE 
        v.pub_id = $1
        and
        o.status = 'accepted';        
       ") .bind(ui)
       .fetch_all(con)
       .await?;

    Ok(orders)
}

pub async fn accept_order(con: &Pool<Postgres>, ui: String) -> Result<String, VendorErr> {
    let accept_query = sqlx::query_file!("sql_queries/accept_order.sql", ui)
        .fetch_one(con)
        .await?;
    
    if accept_query.pub_id.len() == 0 {
        return Err(VendorErr::OrderNotFoud);
    }
     
    Ok(accept_query.pub_id)
}

pub async fn reject_order(con: &Pool<Postgres>, ui: String) -> Result<String, VendorErr> {
    let reject_query = sqlx::query_file!("sql_queries/reject_order.sql", ui)
      .fetch_one(con)
      .await?;

    if reject_query.pub_id.len() == 0 {
        return Err(VendorErr::OrderNotFoud);
    }
     
    Ok(reject_query.pub_id)
}

pub async fn vendor_login(con: &Pool<Postgres>, pw: String) -> Result<String, sqlx::Error> {
    let login = sqlx::query!(
        "
        SELECT pub_id FROM vendors 
        WHERE pw_hash = $1
        ", pw
    )
    .fetch_one(con)
    .await?;

   Ok(login.pub_id) 
}
