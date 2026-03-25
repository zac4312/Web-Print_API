use std::str::FromStr;

use Web_Print_API::{db, service::vendor::{create_vendor, get_vendor}, models::vendors::{Vacancy, Vendor}};
use bigdecimal::BigDecimal;
use sqlx::{self, postgres::types::PgPoint};

#[tokio::test]
async fn create_partner_test() -> Result<(), sqlx::Error> {
    let con = db::connect().await?;

    let pname = "test"; let p_pw = "test"; let email = "a@gmail.com";

    let bw_rate = BigDecimal::from_str("33.33").expect("failed");
    let clrd_rate = BigDecimal::from_str("22.22").expect("failed");
    
    let location = PgPoint{ x: (7.047875), y: (125.451333) }; 

    let shop_status = Vacancy::Busy;

    let partner = Vendor::new(pname.to_string(), p_pw.to_string(), email.to_string() ,bw_rate, clrd_rate, location, shop_status);

    create_vendor(&con, partner).await?; 
    Ok(())
}

#[tokio::test]
async fn list_vendors_test() -> Result<(), sqlx::Error> {
    let con = db::connect().await?;    

    get_vendor(&con).await?;

    Ok(())
}
