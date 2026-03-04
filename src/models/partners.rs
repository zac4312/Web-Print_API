use uuid::Uuid;
use sqlx::postgres::types::PgPoint;

#[derive(sqlx::Type)]
#[sqlx(type_name = "vacancy", rename_all = "lowercase")]
pub enum Availability {
  Available,
  S_busy,
  Busy,
  Closed,
}

pub struct Shop {
    pub shop_id: Uuid,
    pub name: String,
    pub pw_hash: String,
    pub bw_rate: f32,
    pub clrd_rate: f32,
    pub location: PgPoint,
    pub shop_status: Availability,
}

impl Shop {
    pub fn new(name: String, 
                    pw_hash: String,
                    bw_rate: f32,
                    clrd_rate: f32,
                    location: PgPoint,
                    shop_status: Availability) -> Self { 

        Self {  shop_id: Uuid::new_v4(), name, pw_hash, bw_rate, clrd_rate, location, shop_status } }
}
