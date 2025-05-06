use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    repositories::{
        factory::RepositoryFactory, token_repositories::TokenRepository,
        utils_repositories::UtilsRepository,
    },
    utils::utils::{service_response, Response},
};
use serde_json::to_value;

pub fn register_utils_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_initial_utils);
}

#[actix_web::get("/utils/first")]
async fn get_initial_utils(
    repo_factory: web::Data<RepositoryFactory>,
    req: HttpRequest,
) -> impl Responder {
    let utils_repo = repo_factory.create_utils_repository();
    // get the token from the request header
    let token_options = req.headers().get("Token");
    let token = match token_options {
        Some(token) => match token.to_str() {
            Ok(token_str) => token_str,
            Err(_) => return service_response(401, "Unauthorized", "error", None),
        },
        None => return service_response(401, "Unauthorized", "error", None),
    };
    // get the token object by the token
    let token_repo = repo_factory.create_token_repository();
    let token = token_repo.get_token(&token);
    let exists = match token {
        Ok(_) => true,
        Err(_) => return service_response(401, "Unauthorized", "error", None),
    };
    // if not exists unathorized
    if !exists {
        return service_response(401, "Unauthorized", "error", None);
    }
    let utils = utils_repo.fetch_initial_utils();
    let data = match to_value(utils) {
        Ok(val) => Some(val),
        Err(_) => return service_response(500, "Something went wrong", "error", None),
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
