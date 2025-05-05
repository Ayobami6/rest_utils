use crate::config::db::DbPool;
use crate::models::utils::Utils;
use crate::schema::utils;
use crate::schema::utils::dsl::*;
use diesel::prelude::*;

pub trait UtilsRepository {
    fn fetch_initial_utils(&self) -> Utils;
}

// Implementation of the UtilsRepository trait
pub struct UtilsRepositoryImpl {
    pool: DbPool,
}

impl UtilsRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        UtilsRepositoryImpl { pool }
    }
}

impl UtilsRepository for UtilsRepositoryImpl {
    fn fetch_initial_utils(&self) -> Utils {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        let res = utils
            .select(Utils::as_select())
            .limit(1)
            .first::<Utils>(conn)
            .expect("Failed to fetch initial utils");
        res
    }
}
