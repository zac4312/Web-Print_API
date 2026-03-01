mod models;
mod service;
mod routes;
mod db;

use service::user;
use service::partner;

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
    let create_u = user::create_user(&con).await?;
    println!("SUCCESS {:?}", create_u);
    println!("-------------------------------------");

    println!("List Users: ");
    let list_u = user::get_users(&con).await?;
    println!("{:?}", list_u);
    println!("-------------------------------------");
    
    println!("-------------------------------------");
    println!("[ PARTNERS ]");
 
    println!("Create Partner:");
    let create_p = partner::create_partner(&con).await?; 
    println!("SUCCESS {:?}", create_p);
    println!("-------------------------------------");

    println!("List Partners: ");
    let list_p = partner::get_partner(&con).await?;
    println!("{:?}", list_p);
    println!("-------------------------------------");
    
        

println!("-------------------------------------");

    Ok(()) 
}

