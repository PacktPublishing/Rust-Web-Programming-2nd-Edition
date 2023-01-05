use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Algorithm, Header,
                   EncodingKey, DecodingKey, Validation};

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::config::Config;


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
        let key = DecodingKey::from_secret(
            JwToken::get_key().as_ref()
        );
        let token_result = decode::<JwToken>(
            &token, &key,
            &Validation::new(Algorithm::HS256)
        );
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

    fn from_request(req: &HttpRequest,
                    _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str()
                    .unwrap()
                    .to_string();
                let token_result = JwToken::from_token(
                    raw_token
                );
                match token_result {
                    Some(token) => {
                        return ok(token)
                    },
                    None => {
                        let error = ErrorUnauthorized(
                            "token can't be decoded"
                        );
                        return err(error)
                    }
                }
            },
            None => {
                let error = ErrorUnauthorized(
                    "token not in header under key 'token'"
                );
                return err(error)
            }
        }
    }
}


