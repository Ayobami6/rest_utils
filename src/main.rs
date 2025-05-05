mod config;
mod utils;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use config::db::{connect_db, DbPool};
use dotenvy::dotenv;
use utils::utils::get_env_var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let database_url = get_env_var("DATABASE_URL");
    let db_pool: DbPool = connect_db(&database_url);
    println!("Database Connected Successfully");

    println!("Hello, world!");
}
