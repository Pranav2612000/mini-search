use create_routes::create_router;

mod create_routes;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // let searcher = DocSearcher::new("../../index")?;

  let app = create_router();

  let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("Listening on {}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}
