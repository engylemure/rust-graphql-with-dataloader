extern crate dotenv;

//use diesel::prelude::*;
use crate::schema::movies;
use crate::utils::identity::{make_hash, make_salt};
use chrono::*;

#[derive(Identifiable, Queryable, PartialEq, Debug, Associations, Clone)]
#[table_name = "movies"]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub released_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

