

use dataloader::{BatchFn, BatchFuture};

use diesel::prelude::*;
use std::collections::HashMap;
use futures::{future, FutureExt as _FE};
use crate::models::movie_character::MovieCharacter;
use std::collections::hash_map::Entry;
use crate::graphql::SharedMysqlPoolConnection;
use crate::graphql::data_loader::CachedDataLoader;

pub type MovieIdsByCharacterIdDataLoader = CachedDataLoader<i32, Vec<i32>, MovieIdsDataLoaderBatchByCharacterId>;
pub type CharacterIdsByMovieIdDataLoader = CachedDataLoader<i32, Vec<i32>, CharacterIdsDataLoaderBatchByMovieId>;

pub struct MovieIdsDataLoaderBatchByCharacterId {
    pub db:SharedMysqlPoolConnection
}

impl MovieIdsDataLoaderBatchByCharacterId {
    pub fn new(db: SharedMysqlPoolConnection) -> Self {
        Self {
            db
        }
    }
}
pub struct CharacterIdsDataLoaderBatchByMovieId {
    pub db: SharedMysqlPoolConnection
}

impl CharacterIdsDataLoaderBatchByMovieId {
    pub fn new(db: SharedMysqlPoolConnection) -> Self {
        Self {
            db
        }
    }
}

impl BatchFn<i32, Vec<i32>> for MovieIdsDataLoaderBatchByCharacterId {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Vec<i32>, Self::Error> {
        use crate::schema::movie_characters::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        println!("movie_ids_by_character_id_batch keys: {:?}", keys);
        let movie_characters_data: Vec<MovieCharacter> = match movie_characters.filter(character_id.eq_any(keys))
            .load::<MovieCharacter>(conn) {
            Ok(r) => r,
            Err(_e) => Vec::new()
        };
        let mut movie_ids_by_character_id  : HashMap<i32, Vec<i32>> = HashMap::new();
        for movie_character in movie_characters_data {
            match movie_ids_by_character_id.entry(movie_character.character_id) {
                Entry::Occupied(o) => {
                    o.into_mut().push(movie_character.movie_id);
                },
                Entry::Vacant(v) => {
                    v.insert(vec![movie_character.movie_id]);
                }
            }
        }
        future::ready(keys.into_iter().map(|v| {
            match movie_ids_by_character_id.get(v) {
                Some(movie_ids) => movie_ids.clone(),
                None => Vec::new()
            }
        }).collect())
            .unit_error()
            .boxed()
    }
}

impl BatchFn<i32, Vec<i32>> for CharacterIdsDataLoaderBatchByMovieId {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Vec<i32>, Self::Error> {
        use crate::schema::movie_characters::dsl::*;
        println!("character_ids_by_movie_id_batch keys: {:?}", keys);
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        let movie_characters_data: Vec<MovieCharacter> = match movie_characters.filter(movie_id.eq_any(keys))
            .load::<MovieCharacter>(conn) {
            Ok(r) => r,
            Err(_e) => Vec::new()
        };
        let mut movie_ids_by_character_id  : HashMap<i32, Vec<i32>> = HashMap::new();
        for movie_character in movie_characters_data {
            match movie_ids_by_character_id.entry(movie_character.movie_id) {
                Entry::Occupied(o) => {
                    o.into_mut().push(movie_character.character_id);
                },
                Entry::Vacant(v) => {
                    v.insert(vec![movie_character.character_id]);
                }
            }
        }
        future::ready(keys.into_iter().map(|v| {
            match movie_ids_by_character_id.get(v) {
                Some(movie_ids) => movie_ids.clone(),
                None => Vec::new()
            }
        }).collect())
            .unit_error()
            .boxed()
    }
}