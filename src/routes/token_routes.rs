use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use uuid::Uuid;

use crate::{
    models::token::NewTokenDTO,
    repositories::{factory::RepositoryFactory, token_repositories::TokenRepository},
    utils::utils::Response,
};
use chrono::Utc;

use serde_json::to_value;

pub fn register_token_routes(cfg: &mut ServiceConfig) {
    cfg.service(generate_token);
}

// TODO: Make Idempotent
#[actix_web::get("/tokens")]
async fn generate_token(
    repo_factory: web::Data<RepositoryFactory>,
    req: HttpRequest,
) -> impl Responder {
    // generate uuid token
    let token_repo = repo_factory.create_token_repository();
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();
    let ip_string = ip.clone().to_string();
    // lets check the if this ip address has token already
    let token = token_repo.get_token_by_ip(&ip_string);
    let token = match token {
        Ok(token) => Some(to_value(token).unwrap()),
        Err(e) => {
            println!("Error finding token: {:?}", e);
            None
        }
    };
    // this makes the api idempotent
    if token.is_some() {
        let response = Response::new(
            "success".to_string(),
            "Token already exists".to_string(),
            200,
            token,
        );
        let response = response.to_json();
        return HttpResponse::Ok().json(response);
    }

    let token = Uuid::new_v4().to_string();
    let new_token = NewTokenDTO::new(token, ip_string.clone(), Utc::now().naive_utc());

    let created_token = token_repo.create_token(&new_token);
    let data = match created_token {
        Ok(token) => {
            println!("Token created successfully: {:?}", token);
            Some(to_value(token).unwrap())
        }
        Err(e) => {
            println!("Error creating token: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to create token");
        }
    };
    let response = Response::new(
        "success".to_string(),
        "Token generated".to_string(),
        200,
        data,
    );
    let response = response.to_json();
    HttpResponse::Ok().json(response)
}
