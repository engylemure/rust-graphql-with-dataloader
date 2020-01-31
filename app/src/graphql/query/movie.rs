extern crate dotenv;
use crate::errors::{ServiceError};
use crate::graphql::{Context};
use diesel::prelude::*;
use crate::models::movie::Movie;

pub fn movies(context: &Context) -> Result<Vec<Movie>, ServiceError> {
    use crate::schema::movies::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if context.user.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let movies_data = movies.load::<Movie>(conn).expect("");
    Ok(movies_data)
}
