use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::models::transaction_obj::{Size, State};

#[derive(Deserialize)]
pub struct CreateOrder {
    pub copies: u32,
    pub print_size: Size,
    pub color: bool,
    pub user: String,
    pub vendor: String,
    pub file: String,
    pub total: BigDecimal 
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
