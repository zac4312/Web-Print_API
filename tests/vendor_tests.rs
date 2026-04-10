use  Web_Print_API::{db::connect, err::{TransactionErr, VendorErr}, models::vendors::{Vacancy, Vendor}, service::vendor::{self, accept_order, reject_order}};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use sqlx::postgres::types::{PgPoint, };

#[tokio::test]
async fn create_vendor_test() -> Result<(), TransactionErr> {
    let con = connect().await?;
    let name = "name_test".to_string(); let pw_hash = "pwd_test".to_string(); let email = "test@email.com".to_string(); let bw_rate = BigDecimal::from_str("11.11").unwrap(); let clrd_rate = BigDecimal::from_str("22.22").unwrap(); 
    let location = PgPoint { x: (7.133491491244191), y: (125.50872918220560) }; let availability = Vacancy::Available; let brand = "test_brand".to_string(); 

    let vendor = Vendor::new(name, pw_hash, email, bw_rate, clrd_rate, location, availability, brand); 
    
    vendor::create_vendor(&con, vendor).await?;   
    
    Ok(())
}

#[tokio::test]
async fn accept_order_404_test() -> Result<(), VendorErr> {
    let ui = "invalid_id".to_string(); let con = connect().await?; 
    let result = accept_order(&con, &ui).await;

    assert_eq!(result, Err(VendorErr::OrderNotFoud));

    Ok(())
}

#[tokio::test]
async fn reject_order_404_test() -> Result<(), VendorErr> {
    let ui = "invalid_id".to_string(); let con = connect().await?;
    let result = reject_order(&con, &ui).await;
    assert_eq!(result, Err(VendorErr::OrderNotFoud));

    Ok(())
}

#[tokio::test]
async fn reject_order_test() -> Result<(), VendorErr> {
    let ui = "AX8mTlvg".to_string(); let con = connect().await?;
    let result = reject_order(&con, &ui).await;

    assert_eq!(result, Ok(()));

    Ok(())
}

#[tokio::test]
async fn accept_order_test() -> Result<(), VendorErr> {
    let ui = "T0nAa0S7".to_string(); let con = connect().await?; 
    let result = accept_order(&con, &ui).await;

    assert_eq!(result, Ok(()));
     
    Ok(())
}
