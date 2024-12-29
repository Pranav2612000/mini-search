use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use tower_http::trace::ResponseBody;

pub async fn trigger_indexing() -> impl IntoResponse {
  tokio::task::spawn_blocking(|| {
    tokio::runtime::Handle::current().block_on(crawler::start_indexing())
  });

  return Json(serde_json::json!({
    "message": "Indexing triggered"
  }));
}