extern crate dotenv;

use crate::graphql::Context;
use chrono::*;

use crate::models::character::Character;
use crate::models::movie::Movie;
use juniper::FieldError;

#[juniper::graphql_object(description = "A Character Movie", name = "Character", Context = Context)]
impl Character {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        self.name.to_string()
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
    async fn movies(&self, context: &Context) -> Result<Vec<Movie>, FieldError> {
        let movie_ids = context
            .movie_ids_data_loader_by_character_id
            .load(self.id)
            .await;
        match movie_ids {
            Ok(movie_ids) => match context.movie_data_loader_by_id.load_many(movie_ids).await {
                Ok(movies_result) => Ok(movies_result.into_iter().flatten().collect()),
                Err(_) => Ok(Vec::new()),
            },
            Err(_) => Ok(Vec::new()),
        }
    }
}
