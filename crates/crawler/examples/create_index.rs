use crawler::start_indexing;

#[tokio::main]
async fn main() {
  start_indexing().await;
}