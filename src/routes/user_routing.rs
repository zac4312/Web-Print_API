use axum::{Json, Router, http::StatusCode, routing::{get, post}};
use axum_macros::debug_handler;
use http::HeaderMap;

use crate::{db::{self, connect}, dto::user::{CreateUser, CreateUserOut, LoginUser, MadeOrders}, models::users::User, service::user::{create_user, login_user, made_orders}, utils::{get_token, validate_token}};

pub fn route() -> Router {
    Router::new()
        .route("/new_account", post(post_user))
        .route("/login", post(user_login_attempt))
        .route("/orders", get(see_orders))
}

#[debug_handler]
async fn see_orders(header: HeaderMap) -> (StatusCode, Json<Vec<MadeOrders>>) {
    let token = get_token(header).unwrap();
    println!("user_token: {}", token);

    let claim = validate_token(token.to_owned()).unwrap();
    let con = connect().await.unwrap(); let orders = made_orders(&con, claim.claims.sub.to_string()).await.unwrap();
      
    (StatusCode::OK, Json(orders))
}

async fn user_login_attempt(Json(payload): Json<LoginUser>) -> (StatusCode, Json<String>) {
   let con = connect().await.unwrap(); let attempt = login_user(&con, payload.pw).await.unwrap();
    
   (StatusCode::OK, Json(attempt))
}

#[debug_handler]
async fn post_user( Json(payload): Json<CreateUser>) -> (StatusCode ,Json<CreateUserOut>) {
    let user = User::new(payload.name, payload.pw_hash, payload.email);
    let con = db::connect().await.unwrap(); create_user(&con, &user).await.unwrap();
    
    let user_out = CreateUserOut {
            name: user.name,
            email: user.email      
    };

    (StatusCode::CREATED ,Json(user_out))
} 

