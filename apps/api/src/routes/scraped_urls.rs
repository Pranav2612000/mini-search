use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use super::ApiError;

#[derive(Debug, Deserialize)]
pub struct ScrapedUrlsQuery{
    domain: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ScrapedUrlsResponse {
    urls: Vec<String>,
    total: usize,
}

pub async fn scraped_urls(
  Query(params): Query<ScrapedUrlsQuery>
) -> Result<Json<ScrapedUrlsResponse>, ApiError> {
  let searcher = searcher::DocSearcher::new("./index".to_string()).unwrap();

  let limit = params.limit.unwrap_or_else(|| {10});
  let offset = params.offset.unwrap_or_else(|| {0});
  let results = searcher.get_crawled_urls(params.domain, limit, offset).unwrap();

  return Ok(Json(ScrapedUrlsResponse {
    urls: results.urls,
    total: results.total,
  }));
}