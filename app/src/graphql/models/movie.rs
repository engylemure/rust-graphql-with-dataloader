extern crate dotenv;

use crate::graphql::Context;
use chrono::*;

use crate::models::movie::Movie;
use crate::models::character::Character;
use juniper::FieldError;

#[juniper::graphql_object(description = "A Movie", name = "Movie", Context = Context)]
impl Movie {
    fn id(&self) -> i32 { self.id }
    fn name(&self) -> String { self.name.to_string() }
    fn released_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.released_at, Utc)
    }
    fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }
    fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }
    fn deleted(&self) -> bool {
        self.deleted
    }
    async fn characters(&self, context: &Context) -> Result<Vec<Character>, FieldError> {
        let character_ids = context.character_ids_data_loader_by_movie_id.load(self.id).await;
        match  character_ids {
            Ok(characters_ids) => match context.character_data_loader_by_id.load_many(characters_ids).await {
                Ok(characters_result) => Ok(characters_result.into_iter().flatten().collect()),
                Err(_) => Ok(Vec::new())
            },
            Err(_) => Ok(Vec::new())
        }
    }
}