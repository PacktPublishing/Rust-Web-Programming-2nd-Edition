use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::Utc;
use crate::config::Config;


#[derive(Debug)]
pub enum JwTokenError {
    Missing,
    Invalid,
    Expired
}


/// The attributes extracted from the auth token hidding in the header.
/// 
/// # Attributes 
/// * user_id (i32): the ID of the user who's token it belongs to 
/// * language (Option<String>): the language of the user 
/// * class_id (Option<i32>): the ID of the class the user is associated with
/// 
/// # Usage 
/// This can be inserted into a view function like the following example:
/// 
/// ```rust 
/// async fn test(token: jwt::JwToken) -> String {
///     let user_id: i32 = token.user_id;
///     return String::from("it's working")
/// }
/// ```
/// What this does is extract the JWT from the header before it hits the view and passes it as a parameters into the 
/// view.
// #[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}


impl JwToken {

    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
        return key_str.to_owned()
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token
    }

    pub fn new(user_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();

        let expiration = Utc::now()
                                .checked_add_signed(chrono::Duration::minutes(minutes))
                                .expect("valid timestamp")
                                .timestamp();

        return JwToken { user_id, exp: expiration as usize };
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token.as_str(), &key, &Validation::default());
        match token_result {
            Ok(data) => {
                return Ok(data.claims)
            }, 
            Err(error) => {
                let message = format!("{}", error);
                return Err(message)
            }
        }
    }

}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwToken {
    type Error = JwTokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("token") {

            Some(data) => {
                let raw_token = data.to_string();
                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Ok(token) => {
                        return Outcome::Success(token)
                    },
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return Outcome::Failure((Status::BadRequest, JwTokenError::Expired))
                        }
                        return Outcome::Failure((Status::BadRequest, JwTokenError::Invalid))
                    }
                }
            },
            None => {
                return Outcome::Failure((Status::BadRequest, JwTokenError::Missing))
            }
        }
    }
}
