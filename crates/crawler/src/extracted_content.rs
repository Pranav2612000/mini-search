#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub title: String,
    pub content: Vec<String>,
    pub headings: Vec<String>,
    pub code_blocks: Vec<String>,
    pub api_items: Vec<String>,
}