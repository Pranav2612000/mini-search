use tantivy::{collector::TopDocs, query::QueryParser, schema::Field, DateTime, Document, Term};

fn main() {
  let index = tantivy::Index::open_in_dir("../../index").unwrap();
  let schema = index.schema();
  let mut writer = index.writer(50_000_000).unwrap();
  let reader = index.reader().unwrap();

  let mut doc = Document::default();

  doc.add_text(schema.get_field("title").unwrap(), "<title>");
  doc.add_text(schema.get_field("content").unwrap(), "<content>");
  doc.add_text(schema.get_field("url").unwrap(), "https://docs.rs/");

  doc.add_bytes(schema.get_field("url_id").unwrap(), "docs_rs_0");
  doc.add_text(schema.get_field("domain").unwrap(), "docs.rs");
  doc.add_text(schema.get_field("headings").unwrap(), "<heading>");
  doc.add_date(schema.get_field("scraped_at").unwrap(), DateTime::from_timestamp_millis(1735614051069));

  writer.add_document(doc).unwrap();
  writer.commit().unwrap();

  let url_query_parser = QueryParser::for_index(&index, vec![schema.get_field("url").unwrap()]);
  let formatted_url= format!("\"{}\"", "https://docs.rs/");
  let url_query = url_query_parser.parse_query(&formatted_url).unwrap();
  let results = reader.searcher().search(
    &url_query,
    &TopDocs::with_limit(1)
  ).unwrap();

  if results.len() == 0 {
    println!("No results found");
    return;
  }

  // TODO: rescrape if last scrape is less than X duration old
  let doc = reader.searcher().doc(results[0].1).unwrap();
  let url_id= doc.get_first(schema.get_field("url_id").unwrap())
    .and_then(|f| f.as_bytes()).unwrap();

  let term = Term::from_field_bytes(schema.get_field("url_id").unwrap(), &url_id);
  println!("Term {:?}", term);
  writer.delete_term(term);
  writer.commit().unwrap();
  println!("Deleted term from index");
}