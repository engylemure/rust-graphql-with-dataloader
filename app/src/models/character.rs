extern crate dotenv;

//use diesel::prelude::*;
use crate::schema::characters;

use chrono::*;

#[derive(Identifiable, Queryable, PartialEq, Debug, Associations, Clone)]
#[table_name = "characters"]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}
