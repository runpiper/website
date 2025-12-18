use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct MarkdownContent {
    pub frontmatter: Frontmatter,
    pub html: String,
}

#[derive(Debug)]
pub enum MarkdownError {
    NotFound,
    InvalidPath,
    ParseError(String),
}

impl IntoResponse for MarkdownError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MarkdownError::NotFound => (StatusCode::NOT_FOUND, "Page not found"),
            MarkdownError::InvalidPath => (StatusCode::BAD_REQUEST, "Invalid path"),
            MarkdownError::ParseError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.as_str()),
        };
        (status, Html(format!("<h1>{}</h1>", message))).into_response()
    }
}

impl From<std::io::Error> for MarkdownError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::NotFound {
            MarkdownError::NotFound
        } else {
            MarkdownError::ParseError(err.to_string())
        }
    }
}

pub fn parse_markdown_file(file_path: &str) -> Result<MarkdownContent, MarkdownError> {
    // Security: Prevent path traversal
    if file_path.contains("..") || file_path.starts_with('/') {
        return Err(MarkdownError::InvalidPath);
    }

    let content = fs::read_to_string(file_path)?;
    
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&content);
    
    // Parse frontmatter
    let frontmatter: Frontmatter = result
        .data
        .ok_or_else(|| MarkdownError::ParseError("Missing frontmatter".to_string()))?
        .deserialize()
        .map_err(|e| MarkdownError::ParseError(format!("Invalid frontmatter: {}", e)))?;
    
    // Parse markdown to HTML
    let markdown_content = result.content;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    
    let parser = Parser::new_ext(&markdown_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    Ok(MarkdownContent {
        frontmatter,
        html: html_output,
    })
}

pub fn get_blog_post(slug: &str) -> Result<MarkdownContent, MarkdownError> {
    let file_path = format!("content/blog/{}.md", slug);
    parse_markdown_file(&file_path)
}

pub fn get_doc_page(folder: &str, slug: &str) -> Result<MarkdownContent, MarkdownError> {
    let file_path = format!("content/docs/{}/{}.md", folder, slug);
    parse_markdown_file(&file_path)
}

