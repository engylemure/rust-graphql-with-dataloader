

use dataloader::{BatchFn, BatchFuture};
use crate::models::user::User;
use diesel::prelude::*;
use std::collections::HashMap;
use futures::{future, FutureExt as _FE};
use crate::graphql::SharedMysqlPoolConnection;
use crate::graphql::data_loader::CachedDataLoader;

pub type UserByIdDataLoader = CachedDataLoader<i32, Option<User>, UserDataLoaderBatchById>;

pub struct UserDataLoaderBatchById {
    pub db: SharedMysqlPoolConnection
}

impl UserDataLoaderBatchById {
    pub fn new(db: SharedMysqlPoolConnection) -> Self {
        Self {
            db
        }
    }
}



impl BatchFn<i32, Option<User>> for UserDataLoaderBatchById {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Option<User>, Self::Error> {
        use crate::schema::users::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        let users_data: HashMap<i32, User> = match users.filter(id.eq_any(keys))
            .load::<User>(conn) {
            Ok(r) => r,
            Err(_e) => Vec::new()
        }.into_iter().map(|user| (user.id, user)).collect();
        future::ready(keys.into_iter().map(|v| {
            match users_data.get(v) {
                Some(user) => Some(user.clone()),
                None => None
            }
        }).collect())
            .unit_error()
            .boxed()
    }
}