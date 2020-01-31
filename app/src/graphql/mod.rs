pub mod utils;
pub mod models;
pub mod mutation;
pub mod query;
pub mod input;
pub mod dataloader_batchers;
use crate::db::MysqlPooledConnection;
use std::sync::{Arc, Mutex};
//use std::error::Error;
use crate::models::user::UserModel;
use diesel::MysqlConnection;
use diesel::prelude::*;
use crate::errors::ServiceError;
use crate::graphql::mutation::user::*;
use crate::graphql::models::user::*;
use crate::graphql::query::user::*;
use crate::graphql::input::user::*;
use dataloader::{Loader, cached::Loader as CachedLoader, cached};
use crate::graphql::dataloader_batchers::user::UserDataLoaderBatchById;
use std::collections::BTreeMap;

type DataLoader<K, V, B> = Loader<K, V, (), B>;
type CachedDataLoader<K, V, B> = CachedLoader<K, V, (), B, Cache<K, V, B>>;
type Cache<K, V, F> = BTreeMap<K, cached::Item<K, V, (), F>>;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<Mutex<MysqlPooledConnection>>,
    pub user: Option<UserModel>,
    pub user_data_loader_by_id: CachedDataLoader<i32, Option<UserModel>, UserDataLoaderBatchById>
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> Result<Vec<UserModel>, ServiceError> {
        users(context)
    }
    /// Get the authenticated User
    pub fn me(context: &Context) -> Result<UserModel, ServiceError> { me(context) }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, input: RegisterInput) -> Result<Token, ServiceError> {
        register(context, input)
    }
    pub fn login(context: &Context, input: LoginInput) -> Result<Token, ServiceError> {
        login(context, input)
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}

pub fn create_context(user_email: Option<String>, mysql_pool: MysqlPooledConnection) -> Context {
    let db = Arc::new(Mutex::new(mysql_pool));
    Context {
        user_data_loader_by_id: Loader::new(UserDataLoaderBatchById { db: Arc::clone(&db)}).cached(),
        user: find_user(user_email, Arc::clone(&db)),
        db,
    }
}

pub fn find_user(user_email: Option<String>, db: Arc<Mutex<MysqlPooledConnection>>) -> Option<UserModel> {
    use crate::schema::users::dsl::*;
    let conn: &MysqlConnection = &db.lock().unwrap();
    let mut users_data = match users.filter(email.eq(user_email?))
        .load::<UserModel>(conn) {
        Ok(r) => r,
        Err(_e) => Vec::new()
    };
    users_data.pop()
}