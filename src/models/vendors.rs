use sqlx::{postgres::types::PgPoint, prelude::FromRow};
use bigdecimal::BigDecimal;
use crate::utils;

#[derive(sqlx::Type, Debug, PartialEq)]
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
    pub location: PgPoint,
    pub availability: Vacancy,
}

impl Vendor {
    pub fn new(name: String, 
                    pw_hash: String,
                    email: String,
                    bw_rate: BigDecimal,
                    clrd_rate: BigDecimal,
                    location: PgPoint,
                    availability: Vacancy) -> Self { 

        Self {  pub_id: utils::generate_id(8) , name, pw_hash, email, bw_rate, clrd_rate, location, availability }
    }
}

