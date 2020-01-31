extern crate dotenv;

use crate::graphql::Context;
use crate::graphql::utils::generate_uuid_from_str;
use chrono::*;
use uuid::Uuid;
use crate::models::user::User;

#[juniper::graphql_object(description = "A user", name = "User", Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
    fn uuid(&self) -> Option<Uuid> { generate_uuid_from_str(self.uuid.as_str()) }
    fn email(&self) -> String {
        self.email.to_string()
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
}

/// decrypted token JWT and return into login
pub struct Token {
    pub bearer: Option<String>,
    pub user: User,
}

#[juniper::graphql_object(description = "The token object with user information", Context = Context)]
impl Token {
    fn bearer(&self) -> Option<String> {
        Some(self.bearer.as_ref().expect("").to_string())
    }
    fn user(&self) -> &User {
        &self.user
    }
}