use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use crate::utils;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "size", rename_all = "lowercase")]
pub enum Size {
    A4,
    A5,
    A3,
    Letter,
    Legal,
    Tabloid,
}

#[derive(FromRow, Debug, Clone)]
pub struct Order {
    pub copies: u32,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,
    pub status: State,

    pub pub_id: String,
    pub file: String,
    pub target_shop: String,
    pub client: String,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "state", rename_all = "lowercase")]
pub enum State {
    Rejected,
    Paid,
    Claimed,
    Completed,
    Accepted,
    Pending
}
 
impl Order {
    pub fn new(copies: u32,
                    print_size: Size,
                    color: bool, 
                    file: String,
                    total: BigDecimal,
                    target_shop: String,
                    client:  String) -> Self {
       
        Self { pub_id: utils::generate_id(8), copies, print_size, color, total, status: State::Pending ,file ,target_shop, client }
    } 
}

#[derive(FromRow, Debug)]
pub struct FileObj {
    pub file_path: String,
    pub file_size: u32,
    pub mime_type: String,
    pub pub_id: String
}

impl FileObj {
    pub fn new(file_path: String, 
                    file_size: u32, 
                    mime_type: String) -> Self {

        Self { pub_id: utils::generate_id(8) , file_path, file_size, mime_type }
    }
} 






