extern crate dotenv;
use crate::errors::{ServiceError};
use crate::graphql::{Context};
use diesel::prelude::*;
use crate::models::user::UserModel;

pub fn users(context: &Context) -> Result<Vec<UserModel>, ServiceError> {
    use crate::schema::users::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if context.user.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let users_data = users.load::<UserModel>(conn).expect("");
    Ok(users_data)
}

pub fn me(context: &Context) -> Result<UserModel, ServiceError> {
    match context.user.clone() {
        Some(user) => Ok(user),
        None => Err(ServiceError::Unauthorized)
    }
}