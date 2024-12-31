mod doc_extractor;
mod extracted_content;
mod doc_collector;

use std::{sync::Arc, time::Duration};

use config::SITES;
use doc_collector::DocCollector;
use tantivy::{schema::INDEXED, DateOptions};
use voyager::{Collector, CrawlerConfig, RequestDelay};
use futures::StreamExt;

pub async fn start_indexing() {
    println!("Starting indexing...");
    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("content", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("url", tantivy::schema::STORED | tantivy::schema::TEXT);
    schema_builder.add_bytes_field("url_id", tantivy::schema::STORED | tantivy::schema::INDEXED);
    schema_builder.add_text_field("domain", tantivy::schema::STORED);
    schema_builder.add_text_field("headings", tantivy::schema::TEXT | tantivy::schema::STORED);
    
    let date_field_opts = DateOptions::from(INDEXED).set_stored().set_precision(tantivy::schema::DatePrecision::Milliseconds);
    schema_builder.add_date_field("scraped_at", date_field_opts);
    let schema = schema_builder.build();

    let index_dir = tantivy::directory::MmapDirectory::open("./index").unwrap();
    let index = if tantivy::Index::exists(&index_dir).unwrap() {
      tantivy::Index::open(index_dir).unwrap()
    } else {
      tantivy::Index::create_in_dir("./index", schema.clone()).unwrap()
    };

    let config = CrawlerConfig::default()
      .allow_domains_with_delay(
        SITES.iter().map(|site| {
          (site.to_string(), RequestDelay::Random { min: Duration::from_millis(500), max: Duration::from_millis(2000) })
        })
      )
      .respect_robots_txt()
      .max_concurrent_requests(10);

    let doc_collector = DocCollector {
      index: Arc::new(index),
      schema,
      extractors: Arc::new(dashmap::DashMap::new()),
      counter: Arc::new(dashmap::DashMap::new()),
    };

    let mut collector = Collector::new(doc_collector.clone(), config);

    for curr_chunk_sites in SITES.chunks(1) {
      println!("Indexing sites: {:?}", curr_chunk_sites);
      for site in curr_chunk_sites {
        println!("Site: {}", site);
        collector.crawler_mut().visit_with_state(
          &format!("https://{}", site),
          ()
        );
      }
      println!("Visiting sites...");

      while let Some(output) = collector.next().await {
        if let Ok(post) = output {
          // println!("{:?}", post.headings);
        }
      };

      println!("Completed indexing sites: {:?}", curr_chunk_sites);
    }

    println!("Indexing complete!");
}
