mod config;
mod models;
mod repositories;
mod schema;
mod utils;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use config::db::{connect_db, DbPool};
use dotenvy::dotenv;
use repositories::factory::RepositoryFactory;
use utils::utils::get_env_var;
use utils::utils::Response;

#[actix_web::get("/")]
async fn health() -> impl Responder {
    let response = Response::new("success".to_string(), "Server is running".to_string(), None);
    let response = response.to_json();
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let database_url = get_env_var("DATABASE_URL");
    let db_pool: DbPool = connect_db(&database_url);
    println!("🚀 Database Connected Successfully  🚀");

    let factory = RepositoryFactory::new(db_pool.clone());

    let port = get_env_var("PORT");
    let port: u16 = port
        .parse()
        .expect("Invalid port number in environment variable");
    println!("🚀 Starting server on port 🚀 {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(factory.clone()))
            .service(health)
            // .configure(register_user_routes)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
