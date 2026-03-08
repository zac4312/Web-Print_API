mod models;
mod service;
mod routes;
mod db;
mod utils;

use std::str::FromStr;

use crate::{models::vendors, service::{user::{create_user, get_users}, vendor::{create_vendor, get_vendor}}};
use crate::models::{users::User, vendors::Vendor};
use sqlx::postgres::types::PgPoint;
use bigdecimal::BigDecimal;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
println!("-------------------------------------");
println!("--TESTING--");
println!("-------------------------------------");
    
    println!("DB connection: ");
    let con = db::connect().await?;
    println!("SUCCESS"); 
 
println!("-------------------------------------");
    
    println!("[ USERS ]");

    println!("Create User:");
    let name = "TEST"; let pw = "TEST"; let email = "a@gmail.com"; 
    
    let user = User::new(name.to_string(), pw.to_string(), email.to_string());
    let create_u = create_user(&con, user).await?;
    println!("SUCCESS {:?}", create_u);
    println!("-------------------------------------");

    println!("List Users: ");
    let list_u = get_users(&con).await?;
    println!("{:?}", list_u);
    println!("-------------------------------------");
    
    println!("-------------------------------------");
    println!("[ PARTNERS ]");
 
    println!("Create Partner:");
    let pname = "test"; let p_pw = "test"; let email = "a@gmail.com";

    let bw_rate = BigDecimal::from_str("33.33").expect("failed");
    let clrd_rate = BigDecimal::from_str("22.22").expect("failed");
    
    let location = PgPoint { x: (7.047875), y: (125.451333) }; 

    let shop_status = vendors::Vacancy::Busy;

    let partner = Vendor::new(pname.to_string(), p_pw.to_string(), email.to_string() ,bw_rate, clrd_rate, location, shop_status);

    let create_p = create_vendor(&con, partner).await?; 
    println!("SUCCESS {:?}", create_p);
    println!("-------------------------------------");

    println!("List Partners: ");
    let list_p = get_vendor(&con).await?;
    println!("{:?}", list_p);
    println!("-------------------------------------");
    
        

println!("-------------------------------------");

    Ok(()) 
}

