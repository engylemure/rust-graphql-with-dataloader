use crate::models::user::SlimUser;
use crate::utils::jwt::decode_token;
use actix_web::{ http::header, dev, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get(header::AUTHORIZATION);
        let token = match auth {
            Some(header_value) => match header_value.to_str() {
                Ok(value) => Some(value.replace("Bearer", "").trim().to_string()),
                Err(_) => None
            },
            None => None
        };
        match token {
            None => return ok(LoggedUser::default() ),
            Some(token) => match decode_token(&token) {
                Ok(decoded) => return ok(decoded as LoggedUser),
                Err(_) => return ok(LoggedUser::default()),
            },
        }
    }
}