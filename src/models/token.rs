use crate::repositories::token_repositories::TokenRepository;
use crate::schema::tokens;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// querable struct diesel orm model
#[derive(Debug, Queryable, Serialize)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub created_at: Option<NaiveDateTime>,
    pub ip_address: Option<String>,
}

// insertable struct diesel orm model
#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = tokens)]
pub struct NewTokenDTO {
    pub token: String,
    pub ip_address: Option<String>,
}

impl NewTokenDTO {
    pub fn new(token: String, ip_address: String) -> Self {
        Self {
            token,
            ip_address: Some(ip_address),
        }
    }
}
