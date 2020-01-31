use std::sync::{Arc, Mutex};
use crate::db::MysqlPooledConnection;
use dataloader::{BatchFn, BatchFuture};
use crate::models::character::Character;
use diesel::prelude::*;
use std::collections::HashMap;
use futures::{future, FutureExt as _FE};

pub struct CharacterDataLoaderBatchById {
    pub db: Arc<Mutex<MysqlPooledConnection>>
}

impl BatchFn<i32, Option<Character>> for CharacterDataLoaderBatchById {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Option<Character>, Self::Error> {
        use crate::schema::characters::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
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
}