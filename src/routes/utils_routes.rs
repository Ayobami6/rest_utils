use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::{
    repositories::{factory::RepositoryFactory, utils_repositories::UtilsRepository},
    utils::utils::Response,
};
use serde_json::to_value;

pub fn register_utils_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_initial_utils);
}

#[actix_web::get("/utils/first")]
async fn get_initial_utils(repo_factory: web::Data<RepositoryFactory>) -> impl Responder {
    let utils_repo = repo_factory.create_utils_repository();
    let utils = utils_repo.fetch_initial_utils();
    let data = match to_value(utils) {
        Ok(val) => Some(val),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to serialize response"),
    };
    let response = Response::new(
        "success".to_string(),
        "Initial utils fetched".to_string(),
        200,
        data,
    );
    let response = response.to_json();
    HttpResponse::Ok().json(response)
    // HttpResponse::Created().json(response)
}
