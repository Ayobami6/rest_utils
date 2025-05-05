use crate::schema::utils;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct Utils {
    pub id: i32,
    pub name: String,
    pub ai_apikey: String,
    pub created_at: Option<NaiveDateTime>,
}
