extern crate dotenv;
use crate::errors::{ServiceError};
use crate::graphql::{Context};
use diesel::prelude::*;
use crate::models::character::Character;

pub fn characters(context: &Context) -> Result<Vec<Character>, ServiceError> {
    use crate::schema::characters::dsl::*;
    let conn: &MysqlConnection = &context.db.lock().unwrap();
    if context.user.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let characters_data = characters.load::<Character>(conn).expect("");
    Ok(characters_data)
}
