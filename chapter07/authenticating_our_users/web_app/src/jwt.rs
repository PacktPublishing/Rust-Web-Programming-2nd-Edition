use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Algorithm, Header, EncodingKey, DecodingKey, Validation};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::config::Config;


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
    #[serde(with = "ts_seconds")]
    pub minted: DateTime<Utc>
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
        let timestamp = Utc::now();
        return JwToken { user_id, minted: timestamp};
    }

    pub fn from_token(token: String) -> Option<Self> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::new(Algorithm::HS256));
        match token_result {
            Ok(data) => {
                Some(data.claims)
            }, 
            Err(_) => {
                return None
            }
        }
    }

}


impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    /// This gets fired when the JwToken is attached to a request. It fires before the request hits the view.
    /// # Arguments 
    /// The arguments are needed in order for the impl of FromRequest to work. 
    /// 
    /// * req (&HttpRequest): the request that the token is going to be extracted from
    /// * _ (Payload): the payload stream (not used in this function but is needed)
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Some(token) => {
                        return ok(token)
                    },
                    None => {
                        return err(ErrorUnauthorized("token can't be decoded"))
                    }
                }
            },
            None => {
                return err(ErrorUnauthorized("token not in header under key 'token'"))
            }
        }
    }
}

