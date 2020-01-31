extern crate dotenv;

//use diesel::prelude::*;
use crate::schema::users;
use crate::utils::identity::{make_hash, make_salt};
use chrono::*;
use uuid::Uuid;
use crate::graphql::utils::generate_uuid_from_str;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub hash: Vec<u8>,
    pub uuid: String,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    uuid: String,
    hash: Option<Vec<u8>>,
    salt: Option<String>,
    email: Option<&'a String>,
}

impl<'a> Default for NewUser<'a> {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            salt: None,
            email: None,
            hash: None,
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a String, password: &'a String) -> NewUser<'a> {
        let salt = make_salt();
        let hash = make_hash(&password, &salt);
        NewUser {
            salt: Some(salt),
            hash: Some(hash.to_vec()),
            email: Some(&email),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: Option<String>,
    pub uuid: Option<Uuid>
}

impl Default for SlimUser {
    fn default() -> Self {
       Self {
           email: None,
           uuid: None
       }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: Some(user.email),
            uuid: generate_uuid_from_str(user.uuid.as_str())
        }
    }
}
