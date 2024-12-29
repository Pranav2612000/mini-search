use fuzzy_matcher::FuzzyMatcher;
use tantivy::{aggregation::agg_result::AggregationResults, collector::{MultiCollector, TopDocs}, query::{self, QueryParser}, schema::{Field, Schema}, Index, Score};

struct SearchFields {
  title: Field,
  content: Field,
  url: Field,
  domain: Field,
  headings: Field,
  code_blocks: Field,
  api_items: Field,
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
    pub api_items: Vec<String>,
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
      code_blocks: schema.get_field("code_blocks").unwrap(),
      api_items: schema.get_field("api_items").unwrap(),
    };

    let mut query_parser = QueryParser::for_index(&index, vec![
      fields.title,
      fields.content,
      fields.headings,
      fields.code_blocks,
      fields.api_items,
    ]);

    query_parser.set_field_boost(fields.title, 3.0);
    query_parser.set_field_boost(fields.headings, 2.0);
    query_parser.set_field_boost(fields.api_items, 2.5);
    query_parser.set_field_boost(fields.code_blocks, 2.0);

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
    println!("Query {:?}", query);

    let top_docs = TopDocs::with_limit(limit);
    let mut multi_collector = MultiCollector::new();
    let top_docs_handle = multi_collector.add_collector(top_docs);

    let mut results = Vec::new();

    let mut search_results = searcher.search(&query, &multi_collector)?;
    let top_docs = top_docs_handle.extract(&mut search_results);

    println!("top docs {:?}", top_docs);
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
        score,
        api_items: doc.get_first(self.fields.api_items)
        .and_then(|f| f.as_text())
        .map(|text| text.split('\n').map(String::from).collect())
        .unwrap_or_default(),
      };

      results.push(result);
    };

    Ok(results)
  }
}