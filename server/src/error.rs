use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IggyDashboardServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory entry error: {0}")]
    DirEntry(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
}

impl ResponseError for IggyDashboardServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IggyDashboardServerError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({ "error": msg }))
            }
            _ => HttpResponse::InternalServerError().json(json!({ "error": self.to_string() })),
        }
    }
}
