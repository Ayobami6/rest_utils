use crate::schema::utils;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Deserialize)]
#[diesel(table_name = utils)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Utils {
    pub id: i32,
    pub name: String,
    pub ai_apikey: String,
    pub created_at: Option<NaiveDateTime>,
}
