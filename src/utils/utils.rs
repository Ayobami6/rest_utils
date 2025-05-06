use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::any;

pub fn get_env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| {
        panic!(
            "Environment variable {} not set. Please set it and try again.",
            key
        )
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub status_code: i16,
    pub data: Option<serde_json::Value>,
}

impl Response {
    pub fn new(
        status: String,
        message: String,
        status_code: i16,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            status,
            message,
            status_code,
            data,
        }
    }
}
impl Response {
    pub fn to_json(&self) -> serde_json::Value {
        let mut response = serde_json::json!({
            "status": self.status,
            "message": self.message,
            "status_code": self.status_code
        });

        if let Some(data) = &self.data {
            response["data"] = data.clone();
        }
        response
    }
}

pub fn service_response(
    status_code: i16,
    message: &str,
    status: &str,
    data: Option<serde_json::Value>,
) -> HttpResponse {
    let response = Response::new(status.to_string(), message.to_string(), status_code, data);

    match status_code {
        200 => HttpResponse::Ok().json(response.to_json()),
        201 => HttpResponse::Created().json(response.to_json()),
        202 => HttpResponse::Accepted().json(response.to_json()),
        400 => HttpResponse::BadRequest().json(response.to_json()),
        401 => HttpResponse::Unauthorized().json(response.to_json()),
        403 => HttpResponse::Forbidden().json(response.to_json()),
        404 => HttpResponse::NotFound().json(response.to_json()),
        422 => HttpResponse::UnprocessableEntity().json(response.to_json()),
        429 => HttpResponse::TooManyRequests().json(response.to_json()),
        500 => HttpResponse::InternalServerError().json(response.to_json()),
        _ => HttpResponse::InternalServerError().json(response.to_json()),
    }
}
