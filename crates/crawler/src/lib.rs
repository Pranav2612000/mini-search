mod doc_extractor;
mod extracted_content;
mod doc_collector;

use std::sync::{Arc, Mutex};

use doc_collector::DocCollector;
use voyager::{Collector, CrawlerConfig};
use futures::StreamExt;

pub async fn start_indexing() {
    println!("Starting indexing...");
    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("content", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("url", tantivy::schema::STORED);
    schema_builder.add_text_field("domain", tantivy::schema::STORED);
    schema_builder.add_text_field("headings", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("code_blocks", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("api_items", tantivy::schema::TEXT | tantivy::schema::STORED);
    let schema = schema_builder.build();

    let index_dir = tantivy::directory::MmapDirectory::open("../../index").unwrap();
    let index = if tantivy::Index::exists(&index_dir).unwrap() {
      tantivy::Index::open(index_dir).unwrap()
    } else {
      tantivy::Index::create_in_dir("../../index", schema.clone()).unwrap()
    };
    let mut index_writer = index.writer(50_000_000).unwrap();

    let config = CrawlerConfig::default()
      .allow_domains(vec![
          "docs.rs",
          // "doc.rust-lang.org",
      ])
      .respect_robots_txt()
      .max_concurrent_requests(10);

    let mut doc_collector = DocCollector {
      index_writer: Arc::new(Mutex::new(index_writer)),
      schema,
      extractors: Arc::new(dashmap::DashMap::new()),
      counter: Arc::new(dashmap::DashMap::new()),
    };

    let mut collector = Collector::new(doc_collector.clone(), config);

    collector.crawler_mut().visit_with_state(
      "https://docs.rs",
      ()
    );

    while let Some(output) = collector.next().await {
      if let Ok(post) = output {
        println!("{:?}", post.headings);
      }
    };

    doc_collector.commit();
}
