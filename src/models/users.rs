use crate::{utils};
use sqlx::{prelude::FromRow};

#[derive(FromRow, Debug)]
pub struct User {
    pub pub_id: String,
    pub name: String,
    pub pw_hash: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, pw_hash: String, email: String) -> Self {
       Self { 
           pub_id: "usr".to_owned() +  &utils::generate_id(8),
           name,
           email,
           pw_hash
       }
    } 
     
}

