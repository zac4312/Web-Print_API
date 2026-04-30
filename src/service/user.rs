use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::{Pool, Postgres};

use crate::{dto::{jwt::Claims, user::{GetUser, MadeOrders}}, models::users::User, service::transaction::map_user};

pub async fn made_orders(con: &Pool<Postgres>, user: String) -> Result<Vec<MadeOrders>, sqlx::Error> {
    let mut tx = con.begin().await?;
    let user_id = map_user(&mut tx, &user).await?;

    let request = sqlx::query_as::<_, MadeOrders>(
        "
        select o.pub_id as o_pub_id ,o.total, o.status, o.copies, o.print_size, o.color, v.pub_id as v_pub_id, v.brand
        from orders o
        left join vendors v
        on o.for_vendor = v.vendor_id
        where o.for_user = $1
        ") .bind(user_id)
        .fetch_all(con)
        .await?;

    Ok(request)
}

pub async fn login_user(con: &Pool<Postgres>, pw: String) -> Result<String, sqlx::Error> {
    let attempt = sqlx::query!(
    "
    select pub_id from users
    where pw_hash = $1;
    ", pw)
    .fetch_one(con)
    .await?;

    let key = b"secret";
    let claim = Claims {
        sub: attempt.pub_id,
        exp: (Utc::now() + Duration::hours(10)).timestamp() as usize
    };

let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(key)).unwrap();

Ok(token)
}

pub async fn create_user(con: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO users (name, pw_hash, email, pub_id) VALUES ($1, $2, $3, $4)",
            user.name,
            user.pw_hash,
            user.email,
            user.pub_id )
        .execute(con)
        .await?;


    Ok(())
}

pub async fn get_users(con: &Pool<Postgres>) -> Result<Vec<GetUser>, sqlx::Error> {
     let act = sqlx::query_as!(GetUser, "SELECT pub_id, name, email FROM users")
        .fetch_all(con)
        .await?;

     Ok(act) 
}

 

