use crate::graphql::data_loader::CachedDataLoader;
use crate::graphql::SharedMysqlPoolConnection;
use crate::models::movie::Movie;
use dataloader::{BatchFn, BatchFuture};
use diesel::prelude::*;
use futures::{future, FutureExt as _FE};
use std::collections::HashMap;

pub type MovieByIdDataLoader = CachedDataLoader<i32, Option<Movie>, MovieDataLoaderBatchById>;

pub struct MovieDataLoaderBatchById {
    pub db: SharedMysqlPoolConnection,
}

impl MovieDataLoaderBatchById {
    pub fn new(db: SharedMysqlPoolConnection) -> Self {
        Self { db }
    }
}

impl BatchFn<i32, Option<Movie>> for MovieDataLoaderBatchById {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Option<Movie>, Self::Error> {
        use crate::schema::movies::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        let movies_data: HashMap<i32, Movie> =
            match movies.filter(id.eq_any(keys)).load::<Movie>(conn) {
                Ok(r) => r,
                Err(_e) => Vec::new(),
            }
            .into_iter()
            .map(|movie| (movie.id, movie))
            .collect();
        future::ready(
            keys.into_iter()
                .map(|v| match movies_data.get(v) {
                    Some(movie) => Some(movie.clone()),
                    None => None,
                })
                .collect(),
        )
        .unit_error()
        .boxed()
    }
}
