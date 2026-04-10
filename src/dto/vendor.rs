use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};

use crate::models::vendors::Vacancy;

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
    pub gcash: String
}
