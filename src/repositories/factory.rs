use config::db::DbPool;

pub struct RepositoryFactory {
    pool: DbPool,
}

impl RepositoryFactory {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
pub fn create_user_repository(&self) -> TokenRepository {
    TokenRepositoryImpl::new(self.pool.clone())
}
