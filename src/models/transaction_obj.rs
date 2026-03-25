use bigdecimal::BigDecimal;
use sqlx::prelude::FromRow;
use chrono::NaiveDateTime;

use crate::utils;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "size", rename_all = "lowercase")]
pub enum Size {
    A4,
    A5,
    A3,
    Letter,
    Legal,
    Tabloid,
}

#[derive(FromRow, Debug)]
pub struct Order {
    pub copies: u32,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,

    pub pub_id: String,
    pub file: String,
    pub target_shop: String,
    pub client: String,
}

impl Order {
    pub fn new(copies: u32,
                    print_size: Size,
                    color: bool, 
                    file: String,
                    total: BigDecimal,
                    target_shop: String,
                    client:  String) -> Self {
       
        Self { pub_id: utils::generate_id(8), copies, print_size, color, total ,file ,target_shop, client }
    } 
}

#[derive(FromRow, Debug)]
pub struct File {
    pub file_path: String,
    pub file_size: u32,
    pub mime_type: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub pub_id: String
}

impl File {
    pub fn new(file_path: String, 
                    file_size: u32, 
                    mime_type: String,
                    deleted_at: Option<NaiveDateTime> ) -> Self {

        Self { pub_id: utils::generate_id(8) , file_path, file_size, mime_type, deleted_at }
    }
} 






