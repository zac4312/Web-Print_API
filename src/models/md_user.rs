use uuid::Uuid;

pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub pw_hash: String,
}

impl User {
    pub fn new(name: String, pw_hash: String) -> Self {
       Self { 
           user_id: Uuid::new_v4(),
           name,
           pw_hash
       } 
    }     
}

