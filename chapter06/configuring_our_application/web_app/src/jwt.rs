use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok};


pub struct JwToken {
    pub message: String
}


impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload)
                    -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = JwToken{
                    message: data.to_str().unwrap().to_string()
                };
                ok(token)
            },
            None => {
                let token = JwToken{
                    message: String::from("nothing found")
                };
                ok(token)
            }
        }
    }
}

