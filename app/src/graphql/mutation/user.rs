extern crate dotenv;
//use futures::future::Future;
use crate::errors::{ServiceError};
use crate::graphql::Context;
use crate::graphql::utils::{generate_uuid_from_str};
use crate::utils::jwt::create_token;
use diesel::prelude::*;
use validator::{Validate};
use crate::utils::identity::make_hash;
use crate::models::{user::{NewUser, UserModel}};
use crate::graphql::input::user::*;
use crate::graphql::models::user::Token;


type RegisterResult = Result<Token, ServiceError>;
type LoginResult = Result<Token, ServiceError>;

pub fn register(context: &Context, input: RegisterInput) -> RegisterResult {
    use crate::schema::users::dsl::*;
    match input.validate() {
        Ok (_) => {
            let conn: &MysqlConnection = &context.db.lock().unwrap();
            conn.transaction::<_, ServiceError, _>(|| {
                let new_user = NewUser::new(&input.email, &input.password);
                let user_result = diesel::insert_into(users)
                    .values(&new_user)
                    .execute(conn);
                if user_result.is_err() {
                    return Err(ServiceError::from(user_result.err().unwrap()));
                }
                match users.order(id.desc()).first::<UserModel>(conn) {
                    Ok(user) => match create_token(user.email.as_str(), generate_uuid_from_str(&user.uuid).unwrap()) {
                        Ok(token) => Ok(Token {
                            bearer: Some(token),
                            user
                        }),
                        Err(_e) => Err(ServiceError::InternalServerError)
                    },
                    Err(e) => Err(e.into()),
                }
            })
        },
        Err(e) => {
            Err(ServiceError::ValidationError(e.into()))
        }
    }
}

pub fn login(context: &Context, input: LoginInput) -> LoginResult {
    use crate::schema::users::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    let mut items = users
        .filter(email.eq(&input.email))
        .load::<UserModel>(conn)?;
    if let Some(user) = items.pop() {
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