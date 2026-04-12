use axum::{Json, Router, extract::{Multipart, Path}, http::{HeaderMap, StatusCode, header}, routing::{get, post}};
use axum_macros::debug_handler;
use tokio::{fs, io::AsyncWriteExt};
use crate::{db::connect, dto::{file::VendorDownload, vendor::{ CreateVendor, HandlingOrders, OwnedOrders, VendorHome, VendorLogin}}, models::{transaction_obj::State, vendors::{self, Vendor}}, service::vendor::{accept_order, change_availability, create_vendor, get_vendor_home, list_handling_orders, list_orders, reject_order, vendor_login}};

pub fn route() -> Router {
    Router::new()
        .route("/new", post(new_vendor))
        .route("/{pub_id}/add_gcash/", post(add_gcash))
        .route("/download_file", post(download_file))
        .route("/login", post(vendor_login_attempt))
        .route("/{pub_id}/home", get(vendor_home))
        .route("/{pub_id}/change_status", post(change_status))
        .route("/{pub_id}/orders", get(see_orders))
        .route("/accept", post(accept_order_route))
        .route("/reject", post(reject_order_route))
        .route("/{pub_id}/handlingorders", get(handling_orders))
}

async fn handling_orders(vendor: Path<String>) -> (StatusCode, Json<Vec<HandlingOrders>>) { //will fail due to reciept
    let con = connect().await.unwrap(); let orders  = list_handling_orders(&con, &vendor).await.unwrap();
    (StatusCode::OK, Json(orders))
}

#[debug_handler]
async fn accept_order_route(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let con = connect().await.unwrap(); let order = accept_order(&con, payload).await.unwrap();
    (StatusCode::OK, Json(order))
}

async fn reject_order_route(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let con = connect().await.unwrap(); let order = reject_order(&con, payload).await.unwrap();
    (StatusCode::OK, Json(order))
}

#[debug_handler]
async fn see_orders(vendor: Path<String>) -> (StatusCode, Json<Vec<OwnedOrders>>) {
        let con = connect().await.unwrap(); let orders = list_orders(&con, &vendor).await.unwrap();
        (StatusCode::OK, Json(orders))
}

#[debug_handler]
async fn change_status(vendor: Path<String>, Json(new_state): Json<vendors::Vacancy>) -> StatusCode {
    let con = connect().await.unwrap(); change_availability(&con, vendor.to_string(), new_state).await.unwrap();
    StatusCode::OK
}

#[debug_handler]
async fn vendor_home(vendor: Path<String>) -> (StatusCode, Json<Vec<VendorHome>>) {
        let con = connect().await.unwrap(); let location = get_vendor_home(&con, vendor.to_string()).await.unwrap(); 
       (StatusCode::OK, Json(location))
}

async fn vendor_login_attempt(Json(payload): Json<VendorLogin>) -> (StatusCode, Json<String>) {
    let con = connect().await.unwrap(); let login_attempt = vendor_login(&con, payload.pw).await.unwrap();
    (StatusCode::OK, Json(login_attempt))    
}

#[debug_handler]
async fn add_gcash (path: Path<String>, mut gcash_qr: Multipart) -> (StatusCode, Json<String>) {
   if let Some(field) = gcash_qr.next_field().await.unwrap() {
        let data = field.bytes(). await.unwrap();
        let file_path = format!("./vendor_img/{:?}", path);  
        
        let mut file = fs::File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();
        
    return (StatusCode::OK, Json(file_path));
   }else {
       return (StatusCode::PRECONDITION_FAILED, Json("Failed".to_string()));
    };
}

#[debug_handler]
async fn new_vendor(Json(payload): Json<CreateVendor>) -> StatusCode {

    let new_vendor = Vendor::new(payload.name, payload.pw, payload.email, payload.bw_rate, payload.clrd_rate, payload.lat, payload.long, payload.brand);
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

