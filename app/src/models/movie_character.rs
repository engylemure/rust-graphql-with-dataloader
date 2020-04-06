extern crate dotenv;

//use diesel::prelude::*;
use crate::models::character::Character;
use crate::models::movie::Movie;
use crate::schema::movie_characters;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "movie_characters"]
#[belongs_to(Movie)]
#[belongs_to(Character)]
pub struct MovieCharacter {
    pub id: i32,
    pub movie_id: i32,
    pub character_id: i32,
}
