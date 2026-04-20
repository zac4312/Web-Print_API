use bigdecimal::{ToPrimitive};
use sqlx::{Pool, Postgres, Result, Transaction};
use uuid::Uuid;

use crate::{ dto::{order, vendor::ChooseVendor}, err::TransactionErr, models::{ transaction_obj::{FileObj, Order, Size, State}, vendors::Vacancy } };

/*pub async fn get_gcash_path(pub_id: String, con: &Pool<Postgres>) -> Result<String, sqlx::Error>{
    let vendor = sqlx::query!(
        "
        Select gcash from vendors
        where pub_id = $1 
        ", pub_id)
        .fetch_one(con)
        .await?;

    Ok(vendor.gcash)
}*/

pub async fn store_reciept(pub_id: String, reciept: &String, con: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        UPDATE orders
        SET reciept = $1
        WHERE pub_id = $2; 
        ", pub_id, reciept)
        .execute(con)
        .await?;

    Ok(())
}

/*pub async fn choose_vendor(con: &Pool<Postgres>, ui: &String) -> Result<String, TransactionErr>{
    let query = sqlx::query (
    "
    SELECT availability FROM vendors
    where pub_id = $1;
    ")  .bind(ui) 
        .fetch_one(con)
        .await?;

    if query.availability == Vacancy::Closed {return Err(TransactionErr::VendorUnavailable)}

    Ok(query.pub_id)
}*/


pub async fn attach_file(con: &Pool<Postgres>, file: &FileObj) -> Result<(), TransactionErr> {
      sqlx::query_file!("sql_queries/choose_file.sql", file.file_path, file.file_size.to_i64(), file.mime_type, file.pub_id)
            .fetch_one(con)
            .await?;      

    Ok(())
}

pub async fn map_file(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let file_id = sqlx::query!("Select file_id from files where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    Ok(file_id.file_id) 
}

pub async fn map_vendor(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let vendor_id = sqlx::query!("Select vendor_id from vendors where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    Ok(vendor_id.vendor_id) 
}

pub async fn map_user(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let user_id = sqlx::query!("Select user_id from users where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    Ok(user_id.user_id) 
}

pub async fn create_order(con: &Pool<Postgres>, order: &Order) -> Result<(), TransactionErr> {
    let mut tx = con.begin().await?;

    let vendor = map_vendor(&mut tx, &order.target_shop).await?; let file = map_file(&mut tx, &order.file).await?; let user = map_user(&mut tx, &order.client).await?; 

        sqlx::query_file!(
            "sql_queries/create_order.sql",
            order.copies.to_owned().to_i16(), order.print_size.to_owned() as Size, order.color.to_owned(), file, order.pub_id.to_owned(), vendor, user, order.total.to_owned(), order.status.to_owned() as State 
        )

        .execute(tx.as_mut())
        .await?;

    tx.commit().await?;

    Ok(())
}
