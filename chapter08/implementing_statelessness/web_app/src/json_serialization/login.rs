use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}


#[derive(Serialize)]
pub struct LoginResponse {
    token: String
}

