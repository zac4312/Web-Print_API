use axum::{Json, Router, extract::Query, http::StatusCode, routing::{get, post}};
use axum_macros::debug_handler;
use http::HeaderMap;

use crate::{db::{self, connect}, dto::{order::OrderQuery, user::{CreateUser, CreateUserOut, LoginUser, MadeOrders}}, models::{transaction_obj::State, users::User}, service::user::{create_user, login_user, made_orders}, utils::{get_token, validate_token}};

pub fn route() -> Router {
    Router::new()
        .route("/new_account", post(post_user))
        .route("/login", post(user_login_attempt))
        .route("/orders", get(see_orders))
}

#[debug_handler]
async fn see_orders(Query(status): Query<OrderQuery>, header: HeaderMap) -> (StatusCode, Json<Vec<MadeOrders>>) {
    let token = get_token(header).unwrap();
    println!("user_token: {}", token);

    let claim = validate_token(token.to_owned()).unwrap();
    let con = connect().await.unwrap();         
        match status.state {
            Some(State::Pending) => {
                let pending_order = made_orders(&con, claim.claims.sub.to_string(), status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(pending_order));
            },

            Some(State::Paid) => {
                let paid_order = made_orders(&con, claim.claims.sub ,status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(paid_order));
            },

            Some(State::Claimed) => {
                let claimed_orders = made_orders(&con, claim.claims.sub, status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(claimed_orders));
            },

            Some(State::Rejected) => {
                let rejected_orders = made_orders(&con, claim.claims.sub, status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(rejected_orders));
            },

            Some(State::Accepted) => {
                let accepted_orders = made_orders(&con, claim.claims.sub, status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(accepted_orders));
            },

            Some(State::Completed) => {
                let complete_orders = made_orders(&con, claim.claims.sub, status.state.unwrap()).await.unwrap();
                return (StatusCode::OK, Json(complete_orders));
            },

            None => {return (StatusCode::BAD_REQUEST, vec![].into());}
        }
}

async fn user_login_attempt(Json(payload): Json<LoginUser>) -> (StatusCode, Json<String>) {
   let con = connect().await.unwrap(); let attempt = login_user(&con, payload.name ,payload.pw).await.unwrap();
    
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

