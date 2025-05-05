use crate::repositories::token_repositories::TokenRepository;
use crate::schemas::tokens;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// querable struct diesel orm model
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
}

// insertable struct diesel orm model
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = "tokens")]
pub struct NewTokenDTO {
    pub token: String,
}
