extern crate dotenv;
//use futures::future::Future;
use crate::errors::{ServiceError};
use crate::graphql::Context;
use crate::graphql::utils::{generate_uuid_from_str};
use crate::utils::jwt::create_token;
use diesel::prelude::*;
use validator::{Validate};
use crate::utils::identity::make_hash;
use crate::models::{user::{NewUser, User}};
use crate::graphql::input::user::*;
use crate::graphql::models::user::Token;


type RegisterResult = Result<Token, ServiceError>;
type LoginResult = Result<Token, ServiceError>;

pub fn register(context: &Context, input: RegisterInput) -> RegisterResult {
    match input.validate() {
        Ok (_) => {
            let conn: &MysqlConnection = &context.db.lock().unwrap();
            conn.transaction::<_, ServiceError, _>(|| {
                let new_user = NewUser::new(&input.email, &input.password);
                match new_user.save(conn) {
                    Ok(user) => match create_token(user.email.as_str(), generate_uuid_from_str(&user.uuid).unwrap()) {
                        Ok(token) => Ok(Token {
                            bearer: Some(token),
                            user
                        }),
                        Err(_e) => Err(ServiceError::InternalServerError)
                    },
                    Err(e) => Err(e.into())
                }
            })
        },
        Err(e) => {
            Err(ServiceError::ValidationError(e.into()))
        }
    }
}

pub fn login(context: &Context, input: LoginInput) -> LoginResult {
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if let Some(user) = User::by_email(&input.email, conn) {
        if make_hash(&input.password, &user.salt) == user.hash {
            return match generate_uuid_from_str(&user.uuid) {
                Some(user_uuid) => match create_token(input.email.as_str(), user_uuid) {
                    Ok(r) => {
                        Ok(Token {
                            bearer: Some(r),
                            user,
                        })
                    }
                    Err(_e) => Err(ServiceError::Unauthorized),
                },
                None => Err(ServiceError::Unauthorized)
            }
        }
    }
    Err(ServiceError::Unauthorized)
}