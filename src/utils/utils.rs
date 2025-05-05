use std::any;

use serde::{Deserialize, Serialize};

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
