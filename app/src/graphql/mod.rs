pub mod utils;
pub mod models;
pub mod mutation;
pub mod query;
pub mod input;
pub mod data_loader;
use crate::db::MysqlPooledConnection;
use std::sync::{Arc, Mutex};
//use std::error::Error;
use crate::models::user::User;
use crate::models::movie::Movie;
use diesel::MysqlConnection;
use diesel::prelude::*;
use crate::errors::ServiceError;
use crate::graphql::mutation::user::*;
use crate::graphql::models::user::*;
use crate::graphql::query::user::*;
use crate::graphql::input::user::*;
use dataloader::{Loader};
use crate::graphql::data_loader::user::{UserDataLoaderBatchById, UserByIdDataLoader};

use crate::graphql::query::movie::movies;
use crate::graphql::query::character::characters;
use crate::models::character::Character;
use crate::graphql::data_loader::movie::{MovieDataLoaderBatchById, MovieByIdDataLoader};
use crate::graphql::data_loader::character::{CharacterDataLoaderBatchById, CharacterByIdDataLoader};
use crate::graphql::data_loader::movie_character::{MovieIdsDataLoaderBatchByCharacterId, CharacterIdsDataLoaderBatchByMovieId, MovieIdsByCharacterIdDataLoader, CharacterIdsByMovieIdDataLoader};
use crate::graphql::input::movie::MovieFilter;

type SharedMysqlPoolConnection = Arc<Mutex<MysqlPooledConnection>>;

#[derive(Clone)]
pub struct Context {
    pub db: SharedMysqlPoolConnection,
    pub user: Option<User>,
    pub user_data_loader_by_id: UserByIdDataLoader,
    pub movie_data_loader_by_id: MovieByIdDataLoader,
    pub character_data_loader_by_id: CharacterByIdDataLoader,
    pub movie_ids_data_loader_by_character_id: MovieIdsByCharacterIdDataLoader,
    pub character_ids_data_loader_by_movie_id: CharacterIdsByMovieIdDataLoader
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> Result<Vec<User>, ServiceError> {
        users(context)
    }
    pub fn movies(context: &Context, filter: Option<MovieFilter>) -> Result<Vec<Movie>, ServiceError> { movies(context, filter) }
    pub fn characters(context: &Context) -> Result<Vec<Character>, ServiceError> { characters(context) }
    /// Get the authenticated User
    pub fn me(context: &Context) -> Result<User, ServiceError> { me(context) }
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
        user_data_loader_by_id: Loader::new(UserDataLoaderBatchById::new(Arc::clone(&db))).cached(),
        movie_data_loader_by_id: Loader::new(MovieDataLoaderBatchById::new(Arc::clone(&db))).cached(),
        character_data_loader_by_id: Loader::new(CharacterDataLoaderBatchById::new(Arc::clone(&db))).cached(),
        movie_ids_data_loader_by_character_id: Loader::new(MovieIdsDataLoaderBatchByCharacterId::new(Arc::clone(&db))).cached(),
        character_ids_data_loader_by_movie_id: Loader::new(CharacterIdsDataLoaderBatchByMovieId::new(Arc::clone(&db))).cached(),
        user: find_user(user_email, Arc::clone(&db)),
        db,
    }
}

pub fn find_user(user_email: Option<String>, db: Arc<Mutex<MysqlPooledConnection>>) -> Option<User> {
    use crate::schema::users::dsl::*;
    let conn: &MysqlConnection = &db.lock().unwrap();
    let mut users_data = match users.filter(email.eq(user_email?))
        .load::<User>(conn) {
        Ok(r) => r,
        Err(_e) => Vec::new()
    };
    users_data.pop()
}