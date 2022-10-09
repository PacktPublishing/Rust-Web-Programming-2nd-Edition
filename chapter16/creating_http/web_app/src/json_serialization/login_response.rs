use serde::Serialize;


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}
