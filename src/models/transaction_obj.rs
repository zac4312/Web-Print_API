use uuid::Uuid;
use chrono::NaiveDateTime;

pub enum Size {
    A4,
    A5,
    A3,
    Letter,
    Legal,
    Tabloid,
}

pub struct Order {
    pub order_id: Uuid,
    pub copies: u32,
    pub print_size: Size,
    pub color: bool,
    pub dt_stamp: NaiveDateTime,
    
    pub target_shop: Uuid,
    pub client: Uuid,
}

impl Order {
    pub fn new(copies: u32,
                    print_size: Size,
                    color: bool,
                    dt_stamp: NaiveDateTime,
                    target_shop: Uuid,
                    client: Uuid) -> Self {
       
        Self { order_id: Uuid::new_v4(), copies, print_size, color, dt_stamp, target_shop, client }
    } 
}

pub struct File {
    pub file_id: Uuid,
    pub file_path: String,
    pub file_size: u32,
    pub mime_type: String,
    pub deleted_at: Option<NaiveDateTime>,

    pub for_order: Uuid,
}

impl File {
    pub fn new(file_path: String, 
                    file_size: u32, 
                    mime_type: String,
                    deleted_at: Option<NaiveDateTime>,
                    for_order: Uuid) -> Self {

        Self { file_id: Uuid::new_v4(), file_path, file_size, mime_type, deleted_at, for_order }
    }
} 






