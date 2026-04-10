use axum::{Json, Router, extract::{Multipart, Path}, http::{HeaderMap, StatusCode, header}, routing::post};
use axum_macros::debug_handler;
use tokio::{fs, io::AsyncWriteExt};
use crate::{db::connect, dto::{file::VendorDownload, vendor::CreateVendor}, models::vendors::Vendor, service::vendor::create_vendor};

pub fn route() -> Router {
    Router::new()
        .route("/download", post(download_file))
        .route("/new", post(new_vendor))
        .route("/{pub_id}/add_gcash/", post(add_gcash))
        .route("/download_file", post(download_file))
}

async fn vendor_login(Json(payload): Json<VendorLogin>) -> (StatusCode, Json<String>) {
    todo!(/*|- make vendor login dto
            |- make login validation service
            |- write vendor login action 
    */)
}

#[debug_handler]
async fn add_gcash (path: Path<String>, mut gcash_qr: Multipart) -> (StatusCode, Json<String>) {
   if let Some(field) = gcash_qr.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let file_path = format!("./uploads/{:?}", path);  
        
        let mut file = fs::File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();
        
    return (StatusCode::OK, Json(file_path));
   }else {
       return (StatusCode::PRECONDITION_FAILED, Json("Failed".to_string()));
    };
}

#[debug_handler]
async fn new_vendor(Json(payload): Json<CreateVendor>) -> StatusCode {

    let new_vendor = Vendor::new(payload.name, payload.pw, payload.email, payload.bw_rate, payload.clrd_rate, payload.lat, payload.long, payload.brand, payload.gcash);
    let con = connect().await.unwrap(); create_vendor(&con, new_vendor).await.unwrap();

    StatusCode::OK
}

#[debug_handler]
async fn download_file(Json(payload): Json<VendorDownload>) -> (StatusCode, HeaderMap, Vec<u8>) {
        let data = fs::read( payload.file_path).await.unwrap();
        
        let mut headers = HeaderMap::new();
        
        headers.insert(
            header::CONTENT_TYPE,
            "application/octet_stream".parse().unwrap()
        );

        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", payload.pub_id)
                .parse() 
                .unwrap()
        );

    (StatusCode::OK, headers, data)
}

