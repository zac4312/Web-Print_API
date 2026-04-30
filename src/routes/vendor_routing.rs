use axum::{Json, Router, extract::{Multipart, Path}, http::{HeaderMap, StatusCode, header}, routing::{get, post}};
use axum_macros::debug_handler;
use tokio::{fs, io::AsyncWriteExt};
use crate::{db::connect, dto::{file::VendorDownload, vendor::{ CreateVendor, HandlingOrders, OwnedOrders, VendorHome, VendorLogin}}, models::{transaction_obj::State, vendors::{self, Vendor}}, service::vendor::{accept_order, add_gcash, change_availability, create_vendor, get_vendor_home, list_claimed_orders, list_completed_orders, list_handling_orders, list_orders, list_rejected_orders, reject_order, set_o_status_claimed, set_o_status_completed, vendor_login}, utils::{get_token, validate_token}};

pub fn route() -> Router {
    Router::new()
        .route("/new", post(new_vendor)) // DONE
        .route("/add_gcash", post(route_add_gcash)) // DONE
        .route("/download_file", post(download_file)) //DONE
        .route("/login", post(vendor_login_attempt)) // DONE roken
        .route("/home", get(vendor_home)) // DONE token
        .route("/change_status", post(change_status)) // DONE
        .route("/orders", get(see_orders)) //DONE 
        .route("/accept", post(accept_order_route)) //DONE
        .route("/reject", post(reject_order_route)) //DONE
        .route("/handlingorders", get(accepted_orders))
        .route("/rejected_orders", get(rejected_orders)) 
        .route("/claimed_orders", get(claimed_orders))
        .route("/completed_orders", get(completed_orders))
        .route("/set_claimed", post(edit_o_status_claimed))
        .route("/set_completed", post(edit_o_status_completed))
}

async fn edit_o_status_completed(Json(order): Json<String>) -> StatusCode {
    let con = connect().await.unwrap(); set_o_status_completed(&con, order).await.unwrap();
    StatusCode::OK
}

async fn edit_o_status_claimed(Json(order): Json<String>) -> StatusCode {
    let con = connect().await.unwrap(); set_o_status_claimed(&con, order).await.unwrap();
StatusCode::OK
}

async fn completed_orders(header: HeaderMap) -> (StatusCode, Json<Vec<HandlingOrders>>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();

    let con = connect().await.unwrap(); let orders = list_completed_orders(&con, &claim.claims.sub).await.unwrap();
    (StatusCode::OK, Json(orders))
}

async fn claimed_orders(header: HeaderMap) -> (StatusCode, Json<Vec<HandlingOrders>>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();

    let con = connect().await.unwrap(); let orders = list_claimed_orders(&con, &claim.claims.sub).await.unwrap();
    (StatusCode::OK, Json(orders))
}

async fn rejected_orders(header: HeaderMap) -> (StatusCode, Json<Vec<HandlingOrders>>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();

    let con = connect().await.unwrap(); let orders = list_rejected_orders(&con, &claim.claims.sub).await.unwrap();
    (StatusCode::OK, Json(orders))
}

async fn accepted_orders(header: HeaderMap) -> (StatusCode, Json<Vec<HandlingOrders>>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();

    let con = connect().await.unwrap(); let orders  = list_handling_orders(&con, &claim.claims.sub).await.unwrap();
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
async fn see_orders(header: HeaderMap) -> (StatusCode, Json<Vec<OwnedOrders>>) {
    let token = get_token(header).unwrap(); println!("- {}", &token); let claim = validate_token(token).unwrap();
 
    let con = connect().await.unwrap(); let orders = list_orders(&con, &claim.claims.sub).await.unwrap();
        (StatusCode::OK, Json(orders))
}

#[debug_handler]
async fn change_status(header: HeaderMap, Json(new_state): Json<vendors::Vacancy>) -> StatusCode {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();

    let con = connect().await.unwrap(); change_availability(&con, claim.claims.sub, new_state).await.unwrap();
    StatusCode::OK
}

#[debug_handler]
async fn vendor_home(header: HeaderMap) -> (StatusCode, Json<Vec<VendorHome>>) {
        let token = get_token(header).unwrap(); println!("vendor_token: {}", &token); let claim = validate_token(token).unwrap();

        let con = connect().await.unwrap(); let location = get_vendor_home(&con, claim.claims.sub.to_string()).await.unwrap(); 
       (StatusCode::OK, Json(location))
}

async fn vendor_login_attempt(Json(payload): Json<VendorLogin>) -> (StatusCode, Json<String>) {
    let con = connect().await.unwrap(); let login_attempt = vendor_login(&con, payload.pw).await.unwrap();
    (StatusCode::OK, Json(login_attempt))    
}

#[debug_handler]
async fn route_add_gcash (header: HeaderMap, mut gcash_qr: Multipart) -> (StatusCode, Json<String>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();
    let path = claim.claims.sub;

    if let Some(field) = gcash_qr.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap(); 
        let file_path = format!("./vendor_img/{}.png", path.to_string());  
        let mut file = fs::File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();

    let con = connect().await.unwrap(); add_gcash(&con, file_path.to_string(), path).await.unwrap();
        
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
        let data = fs::read(payload.file_path).await.unwrap();
        
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

