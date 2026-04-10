use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateFile {
    pub file_path: String,
    pub file_size: u32,
    pub  mime_type: String,
    pub deleted_at: NaiveDateTime
}

#[derive(Serialize)]
pub struct CreateFileOut {
    pub file_path: String,
    pub file_size: u32,
    pub mime_type: String,
    pub pub_id: String
}
#[derive(Deserialize)]
pub struct VendorDownload {
    pub file_path: String,
    pub pub_id: String
}
