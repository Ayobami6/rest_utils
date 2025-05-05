use crate::config::db::DbPool;

use super::token_repositories::{TokenRepository, TokenRepositoryImpl};
use super::utils_repositories::{UtilsRepository, UtilsRepositoryImpl};

pub struct RepositoryFactory {
    pool: DbPool,
}

impl RepositoryFactory {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub fn create_token_repository(&self) -> impl TokenRepository {
        TokenRepositoryImpl::new(self.pool.clone())
    }
    pub fn create_utils_repository(&self) -> impl UtilsRepository {
        UtilsRepositoryImpl::new(self.pool.clone())
    }
}
