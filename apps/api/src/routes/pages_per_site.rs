use axum::Json;
use serde::Serialize;

use super::ApiError;

#[derive(Debug, Serialize)]
pub struct PagesPerSiteEntry {
  pub domain: String,
  pub count: usize
}

pub async fn pages_per_site() -> Result<Json<Vec<PagesPerSiteEntry>>, ApiError> {
  let searcher = searcher::DocSearcher::new("./index".to_string()).unwrap();
  let results = searcher.get_pages_per_site().unwrap();

  let response = results.iter().map(|e| {
    return PagesPerSiteEntry {
      domain: e.0.to_string(),
      count: e.1
    };
  }).collect();

  return Ok(Json(response));
}