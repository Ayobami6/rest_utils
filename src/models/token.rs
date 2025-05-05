use crate::repositories::token_repositories::TokenRepository;
use crate::schema::tokens;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// querable struct diesel orm model
#[derive(Debug, Queryable)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub created_at: Option<NaiveDateTime>,
}

// insertable struct diesel orm model
#[derive(Deserialize, Insertable)]
#[diesel(table_name = tokens)]
pub struct NewTokenDTO {
    pub token: String,
}

impl NewTokenDTO {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
