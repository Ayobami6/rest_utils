use crate::models::token::{NewTokenDTO, Token};
use crate::schemas::tokens;
use diesel::result::QueryResult;

pub trait TokenRepository {
    fn get_token(&self, token: &str) -> QueryResult<Token>;
    fn create_token(&self, token: &NewTokenDTO) -> QueryResult<Token>;
}

// implementation of the TokenRepository trait
pub struct TokenRepositoryImpl {
    pool: DbPool,
}

impl TokenRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        TokenRepositoryImpl { pool }
    }
}

impl TokenRepository for TokenRepositoryImpl {
    fn get_token(&self, token: &str) -> QueryResult<Token> {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        tokens::table
            .filter(tokens::token.eq(token))
            .first::<Token>(conn)
    }

    fn create_token(&self, token: &NewTokenDTO) -> QueryResult<Token> {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(tokens::table)
            .values(token)
            .get_result(conn)
    }
}
