use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{Error, anyhow};
use tantivy::{collector::TopDocs, query::QueryParser, DateTime, Document, Index, IndexReader, IndexWriter, Term};
use url::Url;
use voyager::{Collector, Crawler, Response, Scraper};

use crate::{doc_extractor::DocExtractor, extracted_content::ExtractedContent};

const NUM_PAGES_PER_SITE: i32 = 100;
const RE_CRAWL_DURATION: i64 = 1000 * 60 * 60 * 24; // 1 day

#[derive(Clone)]
pub struct DocCollector {
  pub index: Arc<Index>,
  pub schema: tantivy::schema::Schema,
  pub extractors: Arc<dashmap::DashMap<String, DocExtractor>>,
  pub counter: Arc<dashmap::DashMap<String, i32>>,
}

fn get_epoch_ms() -> u128 {
  SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis()
}

// Return pure url with hash fragments and query params removed
pub fn parse_url(url: &str) -> String {
  let url = Url::parse(url).unwrap();
  format!("{}://{}{}", url.scheme(), url.host().unwrap(), url.path())
}

impl DocCollector {
  pub fn should_scrape_url (&self, url: &str) -> bool {
    let url_query_parser = QueryParser::for_index(&self.index, vec![self.schema.get_field("url").unwrap()]);
    let url_query = url_query_parser.parse_query(format!("\"{}\"", url).as_str()).unwrap();
    let reader = self.index.reader().unwrap();
    let results = reader.searcher().search(
      &url_query,
      &TopDocs::with_limit(1)
    ).unwrap();

    if results.len() == 0 {
      return true;
    }

    let doc = reader.searcher().doc(results[0].1).unwrap();
    let scraped_at_ms = doc.get_first(self.schema.get_field("scraped_at").unwrap())
      .and_then(|f| f.as_date()).unwrap().into_timestamp_millis();
    let current_ts_ms = get_epoch_ms() as i64;

    if current_ts_ms - scraped_at_ms > RE_CRAWL_DURATION {
      // if site is already crawled, we need to delete the older data as a side effect
      let url_id= doc.get_first(self.schema.get_field("url_id").unwrap())
        .and_then(|f| f.as_bytes()).unwrap();
      let term = Term::from_field_bytes(self.schema.get_field("url_id").unwrap(), &url_id);
      println!("Term {:?}", term);
      let mut writer = self.index.writer(50_000_000).unwrap();

      // Look for a better mechanism to solve this as deleting is expensive
      writer.delete_term(term);
      writer.commit().unwrap();
      return true;
    }

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

      doc.add_bytes(self.schema.get_field("url_id").unwrap(), url.as_str());
      doc.add_text(self.schema.get_field("domain").unwrap(), domain);
      doc.add_text(self.schema.get_field("headings").unwrap(), 
      content.headings.join("\n"));
      doc.add_date(self.schema.get_field("scraped_at").unwrap(), DateTime::from_timestamp_millis(get_epoch_ms() as i64));

      let mut writer = self.index.writer(50_000_000).unwrap();
      writer.add_document(doc)?;
      writer.commit().unwrap();

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