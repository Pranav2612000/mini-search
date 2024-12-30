use create_routes::create_router;

mod create_routes;
mod routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  // let searcher = DocSearcher::new("../../index")?;

  let app = create_router();

  let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("Listening on {}", addr);

  /*
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await.unwrap();
  */

  Ok(app.into())
}
