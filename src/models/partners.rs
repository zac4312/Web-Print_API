use uuid::Uuid;

pub struct GeoLocation {
    pub lat: f64,
    pub lang: f64, 
}

impl GeoLocation {
    pub fn new(lat: f64, lang: f64) -> Self {
        Self { lat, lang }
    }
}

pub enum Availability {
  Available,
  Sbusy,
  Busy,
  Closed,
}

pub struct Shop {
    pub shop_id: Uuid,
    pub name: String,
    pub pw_hash: String,
    pub bw_rate: u32,
    pub clrd_rate: u32,
    pub location: GeoLocation,
    pub shop_status: Availability,
}

impl Shop {
    pub fn new(name: String, 
                    pw_hash: String,
                    bw_rate: u32,
                    clrd_rate: u32,
                    location: GeoLocation,
                    shop_status: Availability) -> Self { 

        Self {  shop_id: Uuid::new_v4(), name, pw_hash, bw_rate, clrd_rate, location, shop_status } }
}
