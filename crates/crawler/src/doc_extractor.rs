use anyhow::Error;
use voyager::scraper::Html;
use spider::{configuration::RedirectPolicy, page::Page, website::Website};
use url::Url;

use crate::extracted_content::ExtractedContent;


#[derive(Debug, Clone)]
pub struct DocExtractor {
  website: Website
}

impl DocExtractor {
  pub fn new(domain: &str) -> Result<Self, Error> {
    let website = Website::new(domain)
      .with_respect_robots_txt(true)
      .with_redirect_policy(RedirectPolicy::Strict)
      .build().unwrap();

    Ok(Self { website })
  }

  pub fn extract_content(&self, html: &Html) -> Result<ExtractedContent, Error> {
    // let spider = Spider::from_website(&self.website);
    // let page = Page::bui(html, url, &spider);
    // let page = Page::new(, client)
    let title_text_ref = html.select(&voyager::scraper::Selector::parse("title").unwrap()).map(|e| e.text().collect()).collect::<Vec<String>>();
    let title = if let Some(txt) = title_text_ref.first() {
      txt
    } else {
      ""
    };
    Ok(ExtractedContent {
      title: title.to_string(),
      content: html.select(&voyager::scraper::Selector::parse("body").unwrap()).map(|e| e.text().collect()).collect(),
      headings: html.select(&voyager::scraper::Selector::parse("h1, h2, h3").unwrap()).map(|e| e.text().collect()).collect(),
    })
  }
}