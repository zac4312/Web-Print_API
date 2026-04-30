use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::transaction_obj::{Size, State};

#[derive(Debug, Serialize)]
pub struct VendorGcash {
    pub gcash: String,
    pub vendor_id: String
} 

#[derive(Deserialize)]
pub struct CreateOrder {
    pub copies: u32,
    pub print_size: Size,
    pub color: bool,
    pub vendor: String,
    pub file: String,
    pub total: BigDecimal,
    pub reciept: Option<String>
}

#[derive(Serialize)]
pub struct CreateOrderOut {
    pub copies: u32,
    pub print_size: Size,
    pub color: bool,
    pub total: BigDecimal,
    pub status: State,
    pub pub_id: String,
    pub user: String,
    pub vendor: String,
    pub file: String,
}
