use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok};


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
pub struct JwToken {
    pub message: String
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
                let token = JwToken{message: data.to_str().unwrap().to_string()};
                ok(token)
            },
            None => {
                let token = JwToken{message: String::from("nothing found")};
                ok(token)
            }
        }
    }
}
