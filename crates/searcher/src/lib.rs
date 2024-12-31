use config::SITES;
use fuzzy_matcher::FuzzyMatcher;
use tantivy::{aggregation::agg_result::AggregationResults, collector::{Count, MultiCollector, TopDocs}, query::{self, QueryParser}, schema::{Field, Schema}, Index, Score};

struct SearchFields {
  title: Field,
  content: Field,
  url: Field,
  domain: Field,
  headings: Field,
  scraped_at: Field,
}

pub struct DocSearcher {
  index: Index,
  schema: Schema,
  query_parser: QueryParser,
  fields: SearchFields
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content_snippet: String,
    pub score: Score,
    pub scraped_at: i64,
}

const CONTEXT_SIZE:usize = 100;
const MAX_LENGTH:usize = 100;

fn generate_snippet(text: &str, query: &str) -> String {
  let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

  let best_position = text
    .split_whitespace()
    .enumerate()
    .filter_map(|(i, word)| {
      matcher.fuzzy_match(word, query).map(|score| (i, score))
    })
    .max_by_key(|(_, score)| *score)
    .map(|(pos, _)| pos);

  if let Some(pos) = best_position {
      let words: Vec<&str> = text.split_whitespace().collect();
      let start = pos.saturating_sub(CONTEXT_SIZE);
      let end = (pos + CONTEXT_SIZE).min(words.len());
      
      let snippet = words[start..end].join(" ");
      if snippet.len() > MAX_LENGTH {
          format!("{}...", &snippet[..MAX_LENGTH])
      } else {
          snippet
      }
  } else {
      // If no match found, return the beginning of the text
      let words: Vec<&str> = text.split_whitespace().take(20).collect();
      format!("{}...", words.join(" "))
  }
}

impl DocSearcher {
  pub fn new(index_path: String) -> tantivy::Result<Self> {
    let index = tantivy::Index::open_in_dir(index_path).unwrap();
    let schema = index.schema();

    let fields = SearchFields {
      title: schema.get_field("title").unwrap(),
      content: schema.get_field("content").unwrap(),
      url: schema.get_field("url").unwrap(),
      domain: schema.get_field("domain").unwrap(),
      headings: schema.get_field("headings").unwrap(),
      scraped_at: schema.get_field("scraped_at").unwrap(),
    };

    let mut query_parser = QueryParser::for_index(&index, vec![
      fields.title,
      fields.content,
      fields.headings,
    ]);

    query_parser.set_field_boost(fields.title, 3.0);
    query_parser.set_field_boost(fields.headings, 2.0);

    Ok(Self {
      index,
      schema,
      query_parser,
      fields
    })
  }

  pub fn search(&self, query_str: &str, limit: usize) -> tantivy::Result<Vec<SearchResult>> {
    let reader = self.index.reader()?;
    let searcher = reader.searcher();

    let query = self.query_parser.parse_query(query_str)?;

    let top_docs = TopDocs::with_limit(limit);
    let mut multi_collector = MultiCollector::new();
    let top_docs_handle = multi_collector.add_collector(top_docs);

    let mut results = Vec::new();

    let mut search_results = searcher.search(&query, &multi_collector)?;
    let top_docs = top_docs_handle.extract(&mut search_results);

    for (score, doc_address) in top_docs {
      let doc = searcher.doc(doc_address)?;

      let result = SearchResult {
        title: doc.get_first(self.fields.title)
        .and_then(|f| f.as_text())
        .unwrap_or_default()
        .to_string(),
        url: doc.get_first(self.fields.url)
        .and_then(|f| f.as_text())
        .unwrap_or_default()
        .to_string(),
        content_snippet: generate_snippet(
          doc.get_first(self.fields.content)
          .and_then(|f| f.as_text())
          .unwrap_or_default(),
          query_str,
        ),
        scraped_at: doc.get_first(self.fields.scraped_at).and_then(|f| f.as_date()).unwrap().into_timestamp_millis(),
        score,
      };

      results.push(result);
    };

    Ok(results)
  }

  // IMPROV: Use facets
  pub fn get_pages_per_site(&self) -> tantivy::Result<Vec<(&str, usize)>> {
    let reader = self.index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(
      &self.index,
      vec![self.schema.get_field("domain").unwrap()]
    );

    let mut result = Vec::new();
    for site in SITES {
      let query = query_parser.parse_query("docs.rs").unwrap();
      let count = searcher.search(&query, &Count)?;

      result.push((site, count));
    }

    return Ok(result);
  }

  pub fn get_crawled_urls(&self, domain: Option<String>, limit: usize, offset: usize) -> tantivy::Result<Vec<String>> {
    let reader = self.index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(
      &self.index,
      vec![self.schema.get_field("domain").unwrap()]
    );

    let query_string = domain.unwrap_or_else(|| {"*".to_string()});
    let query = query_parser.parse_query(&query_string).unwrap();

    let top_docs_collector = TopDocs::with_limit(limit).and_offset(offset);
    let top_docs = searcher.search(&query, &top_docs_collector).unwrap();

    let mut urls = Vec::new();
    for (_, doc_address) in top_docs {
      let doc = searcher.doc(doc_address)?;

      let url = doc.get_first(self.fields.url).and_then(|f| f.as_text()).unwrap_or_default().to_string();

      urls.push(url);
    };

    Ok(urls)
  }
}