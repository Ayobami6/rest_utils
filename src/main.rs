mod config;
mod models;
mod repositories;
mod routes;
mod schema;
mod utils;
use actix_cors::Cors;
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use config::db::{connect_db, DbPool};
use dotenvy::dotenv;
use repositories::factory::RepositoryFactory;
use routes::token_routes::register_token_routes;
use routes::utils_routes::register_utils_routes;
use utils::utils::get_env_var;
use utils::utils::Response;

#[actix_web::get("/")]
async fn health() -> impl Responder {
    let response = Response::new(
        "success".to_string(),
        "Server is running".to_string(),
        200,
        None,
    );
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
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(factory.clone()))
            .service(health)
            .configure(register_utils_routes)
            .configure(register_token_routes)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
