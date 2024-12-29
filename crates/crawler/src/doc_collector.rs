use std::sync::Arc;

use anyhow::{Error, anyhow};
use tantivy::{Document, IndexWriter};
use url::Url;
use voyager::{Collector, Crawler, Response, Scraper};

use crate::{doc_extractor::DocExtractor, extracted_content::ExtractedContent};

const NUM_PAGES_PER_SITE: i32 = 10;

#[derive(Clone)]
pub struct DocCollector {
  pub index_writer: Arc<IndexWriter>,
  pub schema: tantivy::schema::Schema,
  pub extractors: Arc<dashmap::DashMap<String, DocExtractor>>,
  pub counter: Arc<dashmap::DashMap<String, i32>>,
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

    let html = &response.html();
    if let Ok(content) = extractor.extract_content(html) {
      let mut doc = Document::default();
      doc.add_text(self.schema.get_field("title").unwrap(), &content.title);
      doc.add_text(self.schema.get_field("content").unwrap(), &content.content.join("\n"));
      doc.add_text(self.schema.get_field("url").unwrap(), url.as_str());
      doc.add_text(self.schema.get_field("domain").unwrap(), domain);
      doc.add_text(self.schema.get_field("headings").unwrap(), 
      content.headings.join("\n"));
      doc.add_text(self.schema.get_field("code_blocks").unwrap(), 
      content.code_blocks.join("\n"));
      doc.add_text(self.schema.get_field("api_items").unwrap(), 
      content.api_items.join("\n"));

      self.index_writer.add_document(doc)?;

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