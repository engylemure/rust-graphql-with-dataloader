use crate::models::user::SlimUser;
use crate::utils::env::ENV;
use actix_web::HttpResponse;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::convert::From;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    email: String,
    // user uuid
    uuid: Uuid,
}

// struct to get converted to token and back
impl Claims {
    fn with_email(email: &str, uuid: Uuid) -> Self {
        Claims {
            iss: ENV.domain.clone(),
            sub: "auth".into(),
            uuid: uuid.to_owned(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: Some(claims.email),
            uuid: Some(claims.uuid),
        }
    }
}

pub fn create_token(email: &str, uuid: Uuid) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(email, uuid);
    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(ENV.jwt_private_key.as_ref()),
    )
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(ENV.jwt_private_key.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims.into())
    .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}
