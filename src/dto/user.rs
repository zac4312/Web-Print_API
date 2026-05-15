use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::transaction_obj::{Size, State};

#[derive(FromRow, Serialize)]
pub struct MadeOrders {
    pub copies: i16,
    pub print_size: Size,
    pub color: bool, 
    pub total: BigDecimal,
    pub status: State,
    pub o_pub_id: String,
    pub v_pub_id: String,
    pub brand: String,
    pub created_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>
}

#[derive(FromRow, Debug, Serialize)]
pub struct GetUser {
    pub name: String,
    pub email: String,
    pub pub_id: String
}

#[derive(FromRow, Serialize)]
pub struct CreateUserOut {
    pub name: String,
    pub email: String,
}

#[derive(FromRow, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub pw_hash: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub pw: String,
}

pub struct LoginUserOut {
    pub pub_id: String,
} 



