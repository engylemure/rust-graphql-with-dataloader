extern crate dotenv;

//use diesel::prelude::*;
use crate::schema::movie_characters;
use crate::models::movie::Movie;
use crate::models::character::Character;



#[derive(Identifiable, Queryable, Associations)]
#[table_name="movie_characters"]
#[belongs_to(Movie)]
#[belongs_to(Character)]
pub struct MovieCharacter {
    pub id: i32,
    pub movie_id: i32,
    pub character_id: i32
}

