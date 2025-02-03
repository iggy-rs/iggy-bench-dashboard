use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IggyBenchDashboardServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Invalid UUID format: {0}")]
    InvalidUuid(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for IggyBenchDashboardServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IggyBenchDashboardServerError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({ "error": msg }))
            }
            _ => HttpResponse::InternalServerError().json(json!({ "error": self.to_string() })),
        }
    }
}
