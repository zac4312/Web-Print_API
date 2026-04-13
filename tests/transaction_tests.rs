use std::str::FromStr;


use Web_Print_API::{db::{self, connect}, err::TransactionErr, models::transaction_obj::{File, Order, Size }, service::{transaction::{choose_vendor, create_order}, user::map_user}};
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[tokio::test]
async fn available_vendor_transaction_test() -> Result<(), TransactionErr> {
    let con = connect().await?;

    let file_path = "/home/yser/test".to_string(); let file_size = 1200; let mime_type = "jpg".to_string(); let deleted_at = None; 
    let file = File::new(file_path, file_size, mime_type, deleted_at);
    let pub_file_id = &file.pub_id;

    let target_shop = "test_VND_001".to_string();
    let client = "test_USR_001".to_string();
    let ui = &target_shop.clone();
    let from_user = map_user(&con, &client).await?;

    let copies = 1; let print_size = Size::A4; let color = true; let total = BigDecimal::from_str("14.22").unwrap(); 

    let order = Order::new(copies, print_size, color, pub_file_id.to_string(), total, target_shop, client);

    let result = create_order(&con, file, from_user, order, ui).await;

    assert_eq!(result, Ok(()));

    Ok(())
}


#[tokio::test]
async fn closed_vendor_transaction_test() -> Result<(), TransactionErr> {
    let con = connect().await?;

    let file_path = "/home/yser/test".to_string(); let file_size = 1200; let mime_type = "jpg".to_string(); let deleted_at = None; 
    let file = File::new(file_path, file_size, mime_type, deleted_at);
    let pub_file_id = &file.pub_id;

    let target_shop = "test_VND_002".to_string();
    let client = "test_USR_002".to_string();
    let ui = &target_shop.clone();
    let from_user = map_user(&con, &client).await?;

    let copies = 1; let print_size = Size::A4; let color = true; let total = BigDecimal::from_str("14.22").unwrap(); 

    let order = Order::new(copies, print_size, color, pub_file_id.to_string(), total, target_shop, client);

    let result = create_order(&con, file, from_user, order, ui).await;

    assert_eq!(result, Err(TransactionErr::VendorUnavailable));

    Ok(())
}

#[tokio::test]
async fn unkown_vendor_transaction_test() -> Result<(), TransactionErr> {
    let con = connect().await?;

    let file_path = "/home/yser/test".to_string(); let file_size = 1200; let mime_type = "jpg".to_string(); let deleted_at = None; 
    let file = File::new(file_path, file_size, mime_type, deleted_at);
    let pub_file_id = &file.pub_id;

    let target_shop = "unkown_vendor".to_string();
    let client = "test_USR_002".to_string();
    let ui = &target_shop.clone();
    let from_user = map_user(&con, &client).await?;

    let copies = 1; let print_size = Size::A4; let color = true; let total = BigDecimal::from_str("14.22").unwrap(); 

    let order = Order::new(copies, print_size, color, pub_file_id.to_string(), total, target_shop, client);

    let result = create_order(&con, file, from_user, order, ui).await;
    assert_eq!(result, Err(TransactionErr::VendorNotFound));


    Ok(())
}


#[tokio::test]
 async fn choose_vendor_test() -> Result<(), TransactionErr> {
    let con = db::connect().await?;
    let mut tx = con.begin().await?;
    let ui = "test_VND_0003".to_string();

    let result = choose_vendor(&mut tx, &ui).await?;
    let expect = Uuid::new_v4();

    assert_ne!(result, expect);

Ok(())
}
