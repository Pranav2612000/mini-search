use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;

pub mod health_check;
pub mod trigger_indexing;
pub mod search;
pub mod pages_per_site;
pub mod scraped_urls;

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Search error: {0}")]
    SearchError(String),
    #[error("Invalid query: {0}")]
    QueryError(String),
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
      let (status, message) = match self {
          ApiError::SearchError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
          ApiError::QueryError(msg) => (StatusCode::BAD_REQUEST, msg),
          ApiError::Internal(_) => (
              StatusCode::INTERNAL_SERVER_ERROR,
              "Internal server error".to_string(),
          ),
      };

      Json(serde_json::json!({
          "error": message
      }))
      .into_response()
  }
}