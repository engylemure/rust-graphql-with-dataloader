use std::sync::{Arc, Mutex};
use crate::db::MysqlPooledConnection;
use dataloader::{BatchFn, BatchFuture};
use crate::models::user::UserModel;
use diesel::prelude::*;
use std::collections::HashMap;
use futures::{future, FutureExt as _FE};

pub struct UserDataLoaderBatchById {
    pub db: Arc<Mutex<MysqlPooledConnection>>
}

impl BatchFn<i32, Option<UserModel>> for UserDataLoaderBatchById {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Option<UserModel>, Self::Error> {
        use crate::schema::users::dsl::*;
        let conn: &MysqlConnection = &self.db.lock().unwrap();
        let users_data: HashMap<i32, UserModel> = match users.filter(id.eq_any(keys))
            .load::<UserModel>(conn) {
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