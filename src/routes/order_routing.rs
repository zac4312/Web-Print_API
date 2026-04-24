use axum::{Json, Router, extract::{Multipart, Path}, http::{HeaderMap, StatusCode, header}, routing::{get, post}};
use axum_macros::debug_handler;
use chrono::Local;
use tokio::{fs, io::AsyncWriteExt};

use crate::{db::{self, connect}, dto::{file::CreateFileOut, order::{CreateOrder, VendorGcash}, vendor::{ChooseVendor, GetVendors}}, models::transaction_obj::{FileObj, Order}, service::{transaction::{attach_file, create_order, get_gcash_path, store_reciept}, vendor::get_vendor}};

pub fn route() -> Router {
    Router::new()
        .route("/attachfile", post(post_file))
        .route("/listvendors", get(list_vendors))
        .route("/choosevendor", post(route_choose_vendor))
        .route("/createorder", post(post_order))
        .route("/{pub_id}/submit_reciept", post(pay_order))
        .route("/{pub_id}/gcash", get(see_gcash))
}

#[debug_handler]
async fn see_gcash(Path(pub_id): Path<String>) -> (StatusCode, HeaderMap, Vec<u8> ){
        let con = connect().await.unwrap(); let file_path = get_gcash_path(pub_id, &con).await.unwrap();
        let data = fs::read(file_path).await.unwrap();
        
        let mut headers = HeaderMap::new();
        
        headers.insert(
            header::CONTENT_TYPE,
            "image/png".parse().unwrap()
        );
         (StatusCode::OK, headers, data)
}

#[debug_handler]
async fn pay_order(pub_id: Path<String>, mut file: Multipart) -> StatusCode {
     if let Some(field) = file.next_field().await.unwrap() {
        let name = field.file_name().unwrap_or("frdel").to_string(); let file_type = field.content_type().map(|ct| ct.to_string()); let data = field.bytes().await.unwrap();
        let file_path = format!("./reciepts/{}-{}", pub_id.to_string(), name);  
        let mut file = fs::File::create(&file_path).await.unwrap();

        file.write_all(&data).await.unwrap();

        let con = connect().await.unwrap(); store_reciept(pub_id.to_string(), &file_path, &con).await.unwrap(); 
     }

    StatusCode::OK
}

#[debug_handler]
async fn post_file(mut file: Multipart) -> Json<String> {
    if let Some(field) = file.next_field().await.unwrap() {
        let name = field.file_name().unwrap_or("frdel").to_string(); let file_type = field.content_type().map(|ct| ct.to_string()); let data = field.bytes().await.unwrap();

        let file_size = data.len();  let file_path = format!("./uploads/{}",name);  
        
        let mut file = fs::File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();

    let file_obj = FileObj::new(file_path, file_size.try_into().unwrap(), file_type.unwrap_or("failed".to_string()));

    let con = db::connect().await.unwrap(); 
    attach_file(&con, &file_obj).await.unwrap();
        let file_out = CreateFileOut {
            file_size: file_obj.file_size,
            file_path: file_obj.file_path,
            mime_type: file_obj.mime_type,
            pub_id: file_obj.pub_id,
        };
    return Json(file_out.pub_id);
    } else {
        return Json("failed".to_string());
    }     
}

#[debug_handler]
async fn list_vendors() -> Json<Vec<GetVendors>> {
    let con = db::connect().await.unwrap(); let vendors = get_vendor(&con).await.unwrap();
    Json(vendors)
}

#[debug_handler]
async fn route_choose_vendor(Json(payload): Json<ChooseVendor>) -> Json<String> {
let choice = ChooseVendor { pub_id: payload.pub_id };

Json(choice.pub_id)
}
/*async fn route_choose_vendor(Json(payload): Json<ChooseVendor>) -> Json<String> {
    let con = db::connect().await.unwrap(); let choice = choose_vendor(&con, &payload.pub_id).await.unwrap(); 
    
    Json(choice.)
}*/
#[debug_handler]
async fn post_order(Json(payload): Json<CreateOrder>) -> StatusCode {
    
    let order = Order::new(payload.copies, payload.print_size, payload.color, payload.file, payload.total, payload.vendor, payload.user);
    let con = db::connect().await.unwrap(); create_order(&con, &order).await.unwrap();

    StatusCode::OK
}
