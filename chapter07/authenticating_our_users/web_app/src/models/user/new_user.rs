use uuid::Uuid;
use diesel::Insertable;
use bcrypt::{DEFAULT_COST, hash};

use crate::schema::users;


#[derive(Insertable, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}


impl NewUser {
    pub fn new(username: String,
               email: String, password: String) -> NewUser {
        let hashed_password: String = hash(
            password.as_str(), DEFAULT_COST
        ).unwrap();
        let uuid = Uuid::new_v4().to_string();
        return NewUser {
            username,
            email,
            password: hashed_password,
            unique_id: uuid
        }
    }
}
