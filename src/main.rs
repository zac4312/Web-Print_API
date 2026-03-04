mod models;
mod service;
mod routes;
mod db;

use service::{list_accounts, accounts_creation};
use models::{users::User, partners};
use sqlx::postgres::types::PgPoint;

use crate::models::partners::Shop;

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
    let name = "TEST"; let pw = "TEST";
    let user = User::new(name.to_string(), pw.to_string());
    let create_u = accounts_creation::create_user(&con, user).await?;
    println!("SUCCESS {:?}", create_u);
    println!("-------------------------------------");

    println!("List Users: ");
    let list_u = list_accounts::get_users(&con).await?;
    println!("{:?}", list_u);
    println!("-------------------------------------");
    
    println!("-------------------------------------");
    println!("[ PARTNERS ]");
 
    println!("Create Partner:");
    let pname = "test"; let p_pw = "test"; let bw_rate = 33.33; let clrd_rate = 22.22;
    let location = PgPoint { x: (7.047875), y: (125.451333) }; 
    let shop_status = partners::Availability::Busy;
    let partner = Shop::new(pname.to_string(), p_pw.to_string(), bw_rate, clrd_rate, location, shop_status);

    let create_p = accounts_creation::create_partner(&con, partner).await?; 
    println!("SUCCESS {:?}", create_p);
    println!("-------------------------------------");

    println!("List Partners: ");
    let list_p = list_accounts::get_partner(&con).await?;
    println!("{:?}", list_p);
    println!("-------------------------------------");
    
        

println!("-------------------------------------");

    Ok(()) 
}

