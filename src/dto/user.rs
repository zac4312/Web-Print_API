use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::transaction_obj::{Size, State};

#[derive(FromRow, Serialize)]
pub struct MadeOrders {
    pub copies: i16,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,
    pub status: State,
    pub o_pub_id: String,
    pub v_pub_id: String,
    pub brand: String
}

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
