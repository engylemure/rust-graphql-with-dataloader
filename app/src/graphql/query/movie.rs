extern crate dotenv;
use crate::errors::{ServiceError};
use crate::graphql::{Context};
use diesel::prelude::*;
use crate::models::movie::Movie;
use crate::graphql::input::movie::MovieFilter;

pub fn movies(context: &Context, filter: Option<MovieFilter>) -> Result<Vec<Movie>, ServiceError> {
    use crate::schema::movies::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if context.user.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let mut query = movies.into_boxed();
    if let Some(filter) = filter {
        if let Some(id_filter) = filter.id {
            query = query.filter(id.eq(id_filter));
        }
        if let Some(name_filter) = filter.name {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }
    }
    let movies_data = query.load::<Movie>(conn).expect("");
    Ok(movies_data)
}
