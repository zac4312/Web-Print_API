pub mod order_routing;
pub mod user_routing;
pub mod vendor_routing;

use axum::{Json, Router, http::Method, routing::get};
use axum_macros::debug_handler;
use tokio::{net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

use crate::{db, dto::user::GetUser, service::user::get_users};

pub fn route() -> Router {
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any);

    Router::new()
        .nest("/order", order_routing::route())
        .nest("/vendor", vendor_routing::route())
        .nest("/user", user_routing::route())
        .route("/listuser", get(list_users))
        .route("/hello", get(root))
        .layer(cors)
}

pub async fn listener() -> TcpListener {
    TcpListener::bind("0.0.0.0:3001")
        .await.unwrap()
}
#[debug_handler]
async fn root() -> Json<String> {
    Json("Hello World".to_string())
}
#[debug_handler]
async fn list_users() -> Json<Vec<GetUser>> {
   let con = db::connect().await.unwrap(); let get_users = get_users(&con).await.unwrap();
   Json(get_users)
}

