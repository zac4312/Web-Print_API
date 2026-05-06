use axum::{Json, Router, extract::{Multipart, Query}, http::{HeaderMap, StatusCode, header}, routing::{get, post}};
use axum_macros::debug_handler;
use tokio::{fs, io::AsyncWriteExt};
use crate::{db::connect, dto::{file::VendorDownload, order::OrderQuery, vendor::{ CreateVendor, HandlingOrders, OwnedOrders, VendorHome, VendorLogin}}, models::{transaction_obj::State, vendors::{self, Vendor}}, service::vendor::{accept_order, add_gcash, change_availability, create_vendor, get_vendor_home, list_accepted_orders, list_claimed_orders, list_completed_orders, list_orders, list_paid_orders, list_rejected_orders, logout_vendor, reject_order, set_o_status_claimed, set_o_status_completed, set_o_status_paid, vendor_login}, utils::{get_token, validate_token}};

pub fn route() -> Router {
    Router::new()
        .route("/new", post(new_vendor)) // DONE
        .route("/add_gcash", post(route_add_gcash)) // DONE
        .route("/download_file", post(download_file)) //DONE
        .route("/login", post(vendor_login_attempt)) // DONE roken
        .route("/home", get(vendor_home)) // DONE token
        .route("/change_status", post(change_status)) // DONE
        .route("/orders", get(see_pending_orders)) //DONE 
        .route("/accept", post(accept_order_route)) //DONE
        .route("/reject", post(reject_order_route)) //DONE
        .route("/set_claimed", post(edit_o_status_claimed))
        .route("/set_completed", post(edit_o_status_completed))
        .route("/set_paid", post(edit_o_status_paid))
        .route("/handling_orders", get(handling_orders)) 
        .route("/logout", get(logout))
}

#[debug_handler]
async fn logout(header: HeaderMap) -> StatusCode {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();
    let con = connect().await.unwrap(); logout_vendor(&con, &claim.claims.sub).await.unwrap();

    StatusCode::OK
}

async fn handling_orders(Query(status): Query<OrderQuery>, header: HeaderMap) -> (StatusCode, Json<Vec<HandlingOrders>>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap(); let con = connect().await.unwrap();
    
    match status.state {
       Some(State::Paid) => { 
           let paid_order = list_paid_orders(&con, claim.claims.sub).await.unwrap(); 
            (StatusCode::OK, Json(paid_order))
       },

       Some(State::Claimed) => {
           let claimed_orders = list_claimed_orders(&con, &claim.claims.sub).await.unwrap();
            (StatusCode::OK, Json(claimed_orders))
       },

       Some(State::Accepted) => {
           let accepted_orders = list_accepted_orders(&con, &claim.claims.sub).await.unwrap();
            (StatusCode::OK, Json(accepted_orders))
        },
       Some(State::Rejected) => {
           let rejected_orders =  list_rejected_orders(&con, &claim.claims.sub).await.unwrap();
            (StatusCode::OK, Json(rejected_orders))
       },
       Some(State::Completed) => {
           let completed_orders = list_completed_orders(&con, &claim.claims.sub).await.unwrap();
            (StatusCode::OK, Json(completed_orders))
       },
        
       _ => (StatusCode::BAD_REQUEST ,vec![].into())
    }
}


async fn edit_o_status_paid(Json(order): Json<String>) -> StatusCode {
    let con = connect().await.unwrap(); set_o_status_paid(&con, order).await.unwrap();
    StatusCode::OK
}

async fn edit_o_status_completed(Json(order): Json<String>) -> StatusCode {
    let con = connect().await.unwrap(); set_o_status_completed(&con, order).await.unwrap();
    StatusCode::OK
}

async fn edit_o_status_claimed(Json(order): Json<String>) -> StatusCode {
    let con = connect().await.unwrap(); set_o_status_claimed(&con, order).await.unwrap();
StatusCode::OK
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
async fn see_pending_orders(header: HeaderMap) -> (StatusCode, Json<Vec<OwnedOrders>>) {
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
    let con = connect().await.unwrap(); let login_attempt = vendor_login(&con, &payload.name, &payload.pw).await.unwrap();
    (StatusCode::OK, Json(login_attempt))    
}

#[debug_handler]
async fn route_add_gcash (header: HeaderMap, mut gcash_qr: Multipart) -> (StatusCode, Json<String>) {
    let token = get_token(header).unwrap(); let claim = validate_token(token).unwrap();
    let path = claim.claims.sub;

    if let Some(field) = gcash_qr.next_field().await.unwrap() {
        let mime = field.content_type().map(|ct| ct.to_string()).unwrap(); let data = field.bytes().await.unwrap();

        println!("mime: {}", mime);
        if mime != "image/png" {
            return (StatusCode::BAD_REQUEST, Json("failed".to_string()));
        };

        let file_path = format!("./vendor_img/{}.png", path.to_string());  
        let mut file = fs::File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();

    let con = connect().await.unwrap(); add_gcash(&con, file_path.to_string(), path).await.unwrap();
        
    return (StatusCode::OK, Json(file_path));
   }    else {
            return (StatusCode::PRECONDITION_FAILED, Json("Failed".to_string()));
        };
}

#[debug_handler]
async fn new_vendor(Json(payload): Json<CreateVendor>) -> (StatusCode, Json<String>) {

    let new_vendor = Vendor::new(payload.name, payload.pw, payload.email, payload.bw_rate, payload.clrd_rate, payload.lat, payload.long, payload.brand);
    let con = connect().await.unwrap(); create_vendor(&con, &new_vendor).await.unwrap(); let token = vendor_login(&con, &new_vendor.name ,&new_vendor.pw_hash).await.unwrap();

    (StatusCode::OK, Json(token))
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

