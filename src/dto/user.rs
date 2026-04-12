use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize)]
pub struct GetUser {
    pub name: String,
    pub email: String,
    pub pub_id: String
}

#[derive(FromRow, Serialize)]
pub struct CreateUserOut {
    pub name: String,
    pub email: String,
}

#[derive(FromRow, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub pw_hash: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub pw: String
} 
