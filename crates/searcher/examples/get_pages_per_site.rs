use searcher::DocSearcher;

fn main () {
  let doc_searcher = DocSearcher::new("./index".to_string()).unwrap();
  let result = doc_searcher.get_pages_per_site();
  println!("Result: {:?}", result);
}