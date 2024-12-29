use axum::{routing::{get, post}, Router};
use routes::health_check::health_check;
use routes::trigger_indexing::trigger_indexing;
use routes::search::search;

use crate::routes;

#[derive(Clone)]
pub struct ApiState {
}

pub fn create_router() -> Router {
  let state = ApiState {};

  Router::new()
    .route("/api/health", get(health_check))
    .route("/api/index/trigger", post(trigger_indexing))
    .route("/api/search", get(search))
}