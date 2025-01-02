use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use super::ApiError;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    title: String,
    heading: String,
    url: String,
    snippet: String,
    score: f32,
    scraped_at: i64,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    results: Vec<SearchResult>,
    total: usize,
    query: String,
    took_ms: f64,
}

pub async fn search(
  Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
  let start = std::time::Instant::now();

  if params.q.trim().is_empty() {
    return Err(ApiError::QueryError("Query cannot be empty".to_string()));
  }

  let searcher = searcher::DocSearcher::new("./index".to_string()).unwrap();
  let results = searcher.search(params.q.as_str(), 10).unwrap();

  let response = SearchResponse {
    total: results.len(),
    results: results
        .into_iter()
        .map(|r| SearchResult {
            title: r.title,
            heading: r.heading,
            url: r.url,
            snippet: r.content_snippet,
            score: r.score,
            scraped_at: r.scraped_at
        })
        .collect(),
    query: params.q,
    took_ms: start.elapsed().as_secs_f64() * 1000.0,
  };

  return Ok(Json(response));
}