use std::vec;

use axum::{routing::{get, post}, Router};
use axum::http::Method;
use routes::health_check::health_check;
use routes::trigger_indexing::trigger_indexing;
use routes::search::search;
use tower_http::cors::CorsLayer;

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
    .layer(
      CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![axum::http::header::CONTENT_TYPE])
    ) 
}