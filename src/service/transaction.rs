use bigdecimal::{BigDecimal, ToPrimitive};
use sqlx::{Pool, Postgres, Result, Transaction};
use uuid::Uuid;

use crate::{err::TransactionErr, models::{ transaction_obj::{FileObj, Order, Size, State} }};

pub async fn get_total(con: &Pool<Postgres>, pub_id: String, color: bool, copies: BigDecimal) -> Result<BigDecimal, sqlx::Error> {
    if color == true {
       let rate = sqlx::query!(
           "
           Select clrd_rate from vendors
           where pub_id = $1
           ", pub_id
           ) 
           .fetch_one(con).await?;
        
        let total = copies * rate.clrd_rate;
        Ok(total)
    } else {
        let rate = sqlx::query!(
            "
            Select bw_rate from vendors
            where pub_id = $1
            ", pub_id
           ) 
           .fetch_one(con).await?;

        let total = copies * rate.bw_rate;
        Ok(total)
    }
}

pub async fn get_reciept(id: String, con: &Pool<Postgres>) -> Result<String, sqlx::Error> {
    let path = sqlx::query!(
        "
        Select reciept from orders
        Where pub_id = $1 
        ", id
        )
        .fetch_one(con)
        .await?;

    Ok(path.reciept.unwrap_or("path not wokring".to_string()))
} 

pub async fn get_gcash_path(pub_id: String, con: &Pool<Postgres>) -> Result<String, sqlx::Error>{
    let vendor = sqlx::query!(
        "
        Select gcash from vendors
        where pub_id = $1 
        ", pub_id)
        .fetch_one(con)
        .await?;

    
    Ok(vendor.gcash.unwrap())
}

pub async fn store_reciept(pub_id: String, reciept: &String, con: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        UPDATE orders 
        SET reciept = $1
        WHERE pub_id = $2; 
        ", reciept, pub_id)
        .execute(con)
        .await?;

    Ok(())
}

pub async fn attach_file(con: &Pool<Postgres>, file: &FileObj) -> Result<(), TransactionErr> {
      sqlx::query_file!("sql_queries/choose_file.sql", file.file_path, file.file_size.to_i64(), file.mime_type, file.pub_id)
        .fetch_one(con)
            .await?;      

    Ok(())
}

pub async fn map_file(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let file_id = sqlx::query!("Select file_id from files where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    
    println!("file id: {}", &file_id.file_id);

    Ok(file_id.file_id) 
}

pub async fn map_vendor(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let vendor_id = sqlx::query!("Select vendor_id from vendors where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    println!("vendor id: {}", &vendor_id.vendor_id);

    Ok(vendor_id.vendor_id) 
}

pub async fn map_user(tx: &mut Transaction<'_, Postgres>, ui: &String) -> Result<Uuid, sqlx::Error> {
    let user_id = sqlx::query!("Select user_id from users where pub_id = $1", ui).fetch_one(tx.as_mut()).await?;
    println!("user id: {}", user_id.user_id);

    Ok(user_id.user_id) 
}

pub async fn create_order(con: &Pool<Postgres>, order: &Order) -> Result<Uuid, TransactionErr> {
    let mut tx = con.begin().await?;

    let vendor = map_vendor(&mut tx, &order.target_shop).await?; let file = map_file(&mut tx, &order.file).await?; let user = map_user(&mut tx, &order.client).await?; 

        let order_id =  sqlx::query_file!(
            "sql_queries/create_order.sql",
            order.copies.to_owned().to_i16(), order.print_size.to_owned() as Size, order.color.to_owned(), file, order.pub_id.to_owned(), vendor, user, order.total.to_owned(), order.status.to_owned() as State 
        )

        .fetch_one(tx.as_mut())
        .await?;

    tx.commit().await?;

    Ok(order_id.order_id)
}

pub async fn map_order(con: &Pool<Postgres>, pub_id: String) -> Result<Uuid, sqlx::Error> {
    let order = sqlx::query!(
        "
        Select order_id from orders 
        where pub_id = $1
        ", pub_id
    )
        .fetch_one(con)
        .await?;

    Ok(order.order_id)
}

pub async fn date_order(con: &Pool<Postgres>, order_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        Insert Into order_history (order_of)
        values ($1);
        "
    ) .bind(order_id)
        .execute(con)
        .await?;

    Ok(())
}



