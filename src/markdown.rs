use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;

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

pub fn get_doc_index(folder: Option<&str>) -> Result<MarkdownContent, MarkdownError> {
    let file_path = match folder {
        Some(f) => format!("content/docs/{}/index.md", f),
        None => "content/docs/index.md".to_string(),
    };
    parse_markdown_file(&file_path)
}

#[derive(Debug, Clone)]
pub struct BlogPostSummary {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct DocPageSummary {
    pub folder: String,
    pub slug: String,
    pub title: String,
    pub is_index: bool,
}

#[derive(Debug, Clone)]
pub struct DocFolder {
    pub name: String,
    pub display_name: String,
    pub pages: Vec<DocPageSummary>,
}

pub fn list_all_docs() -> Result<Vec<DocFolder>, MarkdownError> {
    let docs_dir = Path::new("content/docs");
    
    if !docs_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut folders: std::collections::HashMap<String, Vec<DocPageSummary>> = std::collections::HashMap::new();
    
    // Walk through all subdirectories
    let entries = fs::read_dir(docs_dir)
        .map_err(|e| MarkdownError::ParseError(format!("Failed to read docs directory: {}", e)))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| MarkdownError::ParseError(e.to_string()))?;
        let path = entry.path();
        
        if path.is_dir() {
            let folder_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| MarkdownError::ParseError("Invalid folder name".to_string()))?
                .to_string();
            
            // Read all .md files in this folder
            let folder_entries = fs::read_dir(&path)
                .map_err(|e| MarkdownError::ParseError(e.to_string()))?;
            
            for file_entry in folder_entries {
                let file_entry = file_entry.map_err(|e| MarkdownError::ParseError(e.to_string()))?;
                let file_path = file_entry.path();
                
                if file_path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let slug = file_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| MarkdownError::ParseError("Invalid filename".to_string()))?
                        .to_string();
                    
                    let is_index = slug == "index";
                    
                    // Parse the file to get the title
                    let content = fs::read_to_string(&file_path)
                        .map_err(|e| MarkdownError::ParseError(e.to_string()))?;
                    
                    let matter = Matter::<YAML>::new();
                    let result = matter.parse(&content);
                    
                    if let Some(data) = result.data {
                        if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                            folders
                                .entry(folder_name.clone())
                                .or_insert_with(Vec::new)
                                .push(DocPageSummary {
                                    folder: folder_name.clone(),
                                    slug,
                                    title: frontmatter.title,
                                    is_index,
                                });
                        }
                    }
                }
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Root level doc file (like index.md)
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| MarkdownError::ParseError("Invalid filename".to_string()))?
                .to_string();
            
            let content = fs::read_to_string(&path)
                .map_err(|e| MarkdownError::ParseError(e.to_string()))?;
            
            let matter = Matter::<YAML>::new();
            let result = matter.parse(&content);
            
            if let Some(data) = result.data {
                if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                    let is_index = slug == "index";
                    folders
                        .entry("_root".to_string())
                        .or_insert_with(Vec::new)
                        .push(DocPageSummary {
                            folder: String::new(),
                            slug,
                            title: frontmatter.title,
                            is_index,
                        });
                }
            }
        }
    }
    
    // Convert to sorted Vec<DocFolder>
    let mut result: Vec<DocFolder> = folders
        .into_iter()
        .map(|(name, mut pages)| {
            // Sort pages: index first, then alphabetically
            pages.sort_by(|a, b| {
                if a.is_index {
                    std::cmp::Ordering::Less
                } else if b.is_index {
                    std::cmp::Ordering::Greater
                } else {
                    a.title.cmp(&b.title)
                }
            });
            
            // Create display name from folder name
            let display_name = name
                .replace('-', " ")
                .replace('_', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            
            DocFolder {
                name,
                display_name,
                pages,
            }
        })
        .collect();
    
    // Sort folders alphabetically, but put _root first
    result.sort_by(|a, b| {
        if a.name == "_root" {
            std::cmp::Ordering::Less
        } else if b.name == "_root" {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });
    
    Ok(result)
}

pub fn list_blog_posts() -> Result<Vec<BlogPostSummary>, MarkdownError> {
    let blog_dir = Path::new("content/blog");
    
    if !blog_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut posts = Vec::new();
    
    let entries = fs::read_dir(blog_dir)
        .map_err(|e| MarkdownError::ParseError(format!("Failed to read blog directory: {}", e)))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| MarkdownError::ParseError(e.to_string()))?;
        let path = entry.path();
        
        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        
        // Get the slug from the filename
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| MarkdownError::ParseError("Invalid filename".to_string()))?
            .to_string();
        
        // Parse the file to get frontmatter
        let content = fs::read_to_string(&path)
            .map_err(|e| MarkdownError::ParseError(e.to_string()))?;
        
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&content);
        
        if let Some(data) = result.data {
            if let Ok(frontmatter) = data.deserialize::<Frontmatter>() {
                posts.push(BlogPostSummary {
                    slug,
                    title: frontmatter.title,
                    description: frontmatter.description,
                    date: frontmatter.date,
                    author: frontmatter.author,
                    tags: frontmatter.tags,
                });
            }
        }
    }
    
    // Sort posts by date (newest first)
    posts.sort_by(|a, b| {
        match (&b.date, &a.date) {
            (Some(date_b), Some(date_a)) => date_b.cmp(date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.title.cmp(&b.title),
        }
    });
    
    Ok(posts)
}

