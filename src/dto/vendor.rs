use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};

use crate::models::{transaction_obj::{Size, State}, vendors::Vacancy};

#[derive(FromRow, Serialize)]
pub struct OwnedOrders {
    pub copies: i16,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,
    pub status: State,
    pub name: String,
    pub file_path: String,
    pub pub_id: String
}

#[derive(FromRow, Serialize)]
pub struct HandlingOrders { 
    pub copies: i16,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,
    pub status: State,
    pub name: String,
    pub file_path: String,
    pub reciept: String
}

#[derive(Serialize, FromRow)]
pub struct GetVendors { 
    pub pub_id: String,
    pub email: String,
    pub bw_rate: BigDecimal,
    pub clrd_rate: BigDecimal,
    pub lat: f64,
    pub long: f64,
    pub availability: Vacancy,
    pub brand: String
}

#[derive(Deserialize, Serialize)]
pub struct ChooseVendor {
    pub pub_id: String 
}

#[derive(Deserialize)]
pub struct CreateVendor {
    pub name: String,
    pub email: String,
    pub bw_rate: BigDecimal,
    pub clrd_rate: BigDecimal,
    pub lat: f64,
    pub long: f64,
    pub brand: String,
    pub pw: String,
}

#[derive(Deserialize)]
pub struct VendorLogin {
    pub pw: String,
    pub username: String
}

#[derive(FromRow, Serialize)]
pub struct VendorHome {
    pub lat: f64,
    pub long: f64
}
