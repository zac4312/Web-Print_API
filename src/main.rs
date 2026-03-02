mod models;
mod service;
mod routes;
mod db;

use service::list_accounts;
use service::accounts_creation;

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
    let create_u = accounts_creation::create_user(&con).await?;
    println!("SUCCESS {:?}", create_u);
    println!("-------------------------------------");

    println!("List Users: ");
    let list_u = list_accounts::get_users(&con).await?;
    println!("{:?}", list_u);
    println!("-------------------------------------");
    
    println!("-------------------------------------");
    println!("[ PARTNERS ]");
 
    println!("Create Partner:");
    let create_p = accounts_creation::create_partner(&con).await?; 
    println!("SUCCESS {:?}", create_p);
    println!("-------------------------------------");

    println!("List Partners: ");
    let list_p = list_accounts::get_partner(&con).await?;
    println!("{:?}", list_p);
    println!("-------------------------------------");
    
        

println!("-------------------------------------");

    Ok(()) 
}

