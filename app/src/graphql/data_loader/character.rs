

use dataloader::{BatchFn, BatchFuture};
use crate::models::character::Character;
use diesel::prelude::*;
use std::collections::HashMap;
use futures::{future, FutureExt as _FE};
use crate::graphql::SharedMysqlPoolConnection;
use crate::graphql::data_loader::CachedDataLoader;

pub type CharacterByIdDataLoader = CachedDataLoader<i32, Option<Character>, CharacterDataLoaderBatchById>;

pub struct CharacterDataLoaderBatchById {
    pub db: SharedMysqlPoolConnection
}

impl CharacterDataLoaderBatchById {
    pub fn new(db: SharedMysqlPoolConnection) -> Self {
        Self {
            db
        }
    }
}

impl BatchFn<i32, Option<Character>> for CharacterDataLoaderBatchById {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Option<Character>, Self::Error> {
        use crate::schema::characters::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        println!("character_by_id_batch keys: {:?}", keys);
        let characters_data: HashMap<i32, Character> = match characters.filter(id.eq_any(keys))
            .load::<Character>(conn) {
            Ok(r) => r,
            Err(_e) => Vec::new()
        }.into_iter().map(|character| (character.id, character)).collect();
        future::ready(keys.into_iter().map(|v| {
            match characters_data.get(v) {
                Some(character) => Some(character.clone()),
                None => None
            }
        }).collect())
            .unit_error()
            .boxed()
    }

    #[inline(always)]
    fn max_batch_size(&self) -> usize {
        500
    }
}