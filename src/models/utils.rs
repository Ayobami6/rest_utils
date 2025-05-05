use crate::schema::utils;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Utils {
    pub id: i32,
    pub name: String,
    pub ai_apikey: String,
    pub created_at: chrono::NaiveDateTime,
}
