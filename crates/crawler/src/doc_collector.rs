use std::sync::{Arc, Mutex};

use anyhow::{Error, anyhow};
use tantivy::{collector::{FilterCollector, TopDocs}, query::{Query, QueryParser}, Document, IndexReader, IndexWriter};
use url::Url;
use voyager::{Collector, Crawler, Response, Scraper};

use crate::{doc_extractor::DocExtractor, extracted_content::ExtractedContent};

const NUM_PAGES_PER_SITE: i32 = 100;

#[derive(Clone)]
pub struct DocCollector {
  pub index_writer: Arc<Mutex<IndexWriter>>,
  pub index_reader: Arc<IndexReader>,
  pub url_query_parser: Arc<QueryParser>,
  pub schema: tantivy::schema::Schema,
  pub extractors: Arc<dashmap::DashMap<String, DocExtractor>>,
  pub counter: Arc<dashmap::DashMap<String, i32>>,
}

// Return pure url with hash fragments and query params removed
pub fn parse_url(url: &str) -> String {
  let url = Url::parse(url).unwrap();
  format!("{}://{}{}", url.scheme(), url.host().unwrap(), url.path())
}

impl DocCollector {
  pub fn commit(&mut self) {
    self.index_writer.lock().unwrap().commit().unwrap();
  }

  pub fn should_scrape_url (&self, url: &str) -> bool {
    let url_query = self.url_query_parser.parse_query(format!("\"{}\"", url).as_str()).unwrap();
    let results = self.index_reader.searcher().search(
      &url_query,
      &TopDocs::with_limit(1)
    ).unwrap();

    if results.len() == 0 {
      return true;
    }

    // TODO: rescrape if last scrape is less than X duration old
    return false;
  }
}

impl Scraper for DocCollector {
  type Output = ExtractedContent;
  type State = ();

  fn scrape(&mut self, response: Response<Self::State>, crawler: &mut Crawler<Self>) -> Result<Option<Self::Output>, Error> {

    let url = response.request_url.clone();

    let domain = url.domain().ok_or_else(|| anyhow!("No domain found"))?;

    let extractor = self.extractors.entry(domain.to_string())
    .or_insert_with(|| DocExtractor::new(domain).unwrap())
    .clone();

    let counter = self.counter.entry(domain.to_string())
    .or_insert_with(|| 0)
    .clone();

    if counter > NUM_PAGES_PER_SITE {
      return Ok(None);
    }

    if !self.should_scrape_url(parse_url(url.as_str()).as_str()) {
      println!("Ignoring {:?} as it has been scraped recently", url.as_str());
      return Ok(None);
    }

    let html = &response.html();
    if let Ok(content) = extractor.extract_content(html) {
      let mut doc = Document::default();
      println!("title: {} url: {}", content.title, url.as_str());

      doc.add_text(self.schema.get_field("title").unwrap(), &content.title);
      doc.add_text(self.schema.get_field("content").unwrap(), &content.content.join("\n"));
      doc.add_text(self.schema.get_field("url").unwrap(), parse_url(url.as_str()).as_str());
      doc.add_text(self.schema.get_field("domain").unwrap(), domain);
      doc.add_text(self.schema.get_field("headings").unwrap(), 
      content.headings.join("\n"));
      doc.add_text(self.schema.get_field("code_blocks").unwrap(), 
      content.code_blocks.join("\n"));
      doc.add_text(self.schema.get_field("api_items").unwrap(), 
      content.api_items.join("\n"));

      self.index_writer.lock().unwrap().add_document(doc)?;
      self.index_writer.lock().unwrap().commit().unwrap();

      let links = html.select(&voyager::scraper::Selector::parse("a").unwrap())
        .map(|e| e.value().attr("href").unwrap_or_default())
        .collect::<Vec<_>>();
      for link in links {
        if let Ok(url) = Url::parse(link) {
          crawler.visit_with_state(url, ());
        }
        
        if let Ok(url) = url.join(link) {
          crawler.visit_with_state(url, ());
        }
      }

      self.counter.alter(&domain.to_string(), |_, i| { i + 1 });

      return Ok(Some(content));
    }

    return Ok(None);
  }
}