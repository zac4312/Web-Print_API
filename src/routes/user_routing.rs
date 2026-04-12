use axum::{Json, Router, http::StatusCode, routing::post};
use axum_macros::debug_handler;

use crate::{db::{self, connect}, dto::user::{CreateUser, CreateUserOut, LoginUser}, models::users::User, service::user::{create_user, login_user}};

pub fn route() -> Router {
    Router::new()
        .route("/new_account", post(post_user))
        .route("/login", post(user_login_attempt))
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

