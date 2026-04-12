use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};
use bigdecimal::BigDecimal;
use crate::{utils};

#[derive(sqlx::Type, Debug, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "vacancy", rename_all = "snake_case")]
pub enum Vacancy{
    Available,
    SBusy,
    Busy,
    Closed,
}

#[derive(FromRow, Debug)]
pub struct Vendor {
    pub pub_id: String,
    pub name: String,
    pub pw_hash: String,
    pub email: String,
    pub bw_rate: BigDecimal,
    pub clrd_rate: BigDecimal,
    pub lat: f64,
    pub long: f64,
    pub availability: Vacancy,
    pub brand: String,
    pub gcash: Option<String>
}

impl Vendor {
    pub fn new(name: String, 
                    pw_hash: String,
                    email: String,
                    bw_rate: BigDecimal,
                    clrd_rate: BigDecimal,
                    lat: f64,
                    long: f64,
                    brand: String) -> Self { 

        Self {  pub_id: utils::generate_id(8) , name, pw_hash, email, bw_rate, clrd_rate, lat, long, availability: Vacancy::Available, brand, gcash: None }
    }
}

