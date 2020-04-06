extern crate dotenv;
use crate::errors::ServiceError;
use crate::graphql::Context;
use crate::models::user::User;
use diesel::prelude::*;

pub fn users(context: &Context) -> Result<Vec<User>, ServiceError> {
    use crate::schema::users::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if context.user.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let users_data = users.load::<User>(conn).expect("");
    Ok(users_data)
}

pub fn me(context: &Context) -> Result<User, ServiceError> {
    match context.user.clone() {
        Some(user) => Ok(user),
        None => Err(ServiceError::Unauthorized),
    }
}
