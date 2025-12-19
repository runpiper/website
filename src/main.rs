use askama::Template;
use axum::{
    routing::get,
    Router,
    Json,
    extract::Path,
    extract::Request,
    response::{Response, IntoResponse},
    http::{StatusCode, header, HeaderValue},
};
use serde::Serialize;
use std::io::Write;
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
};
use tower::ServiceBuilder;

mod seo;
use seo::SeoMeta;

mod markdown;
use markdown::{get_blog_post, get_doc_page, get_doc_index, list_blog_posts, list_all_docs, BlogPostSummary, DocFolder, MarkdownError};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    // SEO fields
    site_name: &'static str,
    page_title: &'static str,
    meta_description: &'static str,
    meta_keywords: &'static str,
    canonical_url: String,
    base_url: &'static str,
    og_image: String,
    current_year: u16,
    
    // Page content
    heading: &'static str,
    subheading: &'static str,
    welcome_text: &'static str,
}

impl HomeTemplate {
    fn new(seo: SeoMeta) -> Self {
        Self {
            site_name: seo.site_name,
            page_title: "Home",
            meta_description: "RunPiper is an open-source, enterprise-grade AI agent runtime built in Rust. Deploy thousands of agents with one click—self-hosted or on RunPiper Cloud.",
            meta_keywords: "AI agents, agent runtime, Rust, enterprise AI, open source, agent deployment, agent orchestration",
            canonical_url: seo.base_url.to_string(),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            
            heading: "The Runtime for Production AI Agents",
            subheading: "Open-source. Built in Rust. One-click deploy to RunPiper Cloud or self-host.",
            welcome_text: "Prototyping AI agents is easy—running them reliably at scale is not. RunPiper handles the hard parts: memory-safe execution, sub-millisecond cold starts, and rock-solid stability for thousands of concurrent agents. Build with any framework, deploy anywhere.",
        }
    }
}

async fn home() -> HomeTemplate {
    HomeTemplate::new(SeoMeta::default())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

// Breadcrumb item
#[derive(Clone)]
struct BreadcrumbItem {
    label: String,
    url: String,
}

// Blog template
#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    // SEO fields
    site_name: &'static str,
    page_title: String,
    meta_description: String,
    meta_keywords: &'static str,
    canonical_url: String,
    base_url: &'static str,
    og_image: String,
    current_year: u16,
    
    // Blog content
    title: String,
    date: Option<String>,
    author: Option<String>,
    tags: Option<Vec<String>>,
    content: String,
    
    // Breadcrumbs
    breadcrumbs: Vec<BreadcrumbItem>,
}

impl BlogTemplate {
    fn new(slug: &str, title: String, description: Option<String>, date: Option<String>, author: Option<String>, tags: Option<Vec<String>>, content: String) -> Self {
        let seo = SeoMeta::default();
        let meta_desc = description.unwrap_or_else(|| format!("{} - {}", title, seo.site_name));
        
        // Build breadcrumbs: Blog > [Post Title]
        let breadcrumbs = vec![
            BreadcrumbItem {
                label: "Blog".to_string(),
                url: "/blog".to_string(),
            },
            BreadcrumbItem {
                label: title.clone(),
                url: format!("/blog/{}", slug),
            },
        ];
        
        Self {
            site_name: seo.site_name,
            page_title: title.clone(),
            meta_description: meta_desc,
            meta_keywords: "blog, AI agents, runpiper",
            canonical_url: format!("{}/blog/{}", seo.base_url, slug),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            title,
            date,
            author,
            tags,
            content,
            breadcrumbs,
        }
    }
}

// Docs template
#[derive(Template)]
#[template(path = "docs.html")]
struct DocsTemplate {
    // SEO fields
    site_name: &'static str,
    page_title: String,
    meta_description: String,
    meta_keywords: &'static str,
    canonical_url: String,
    base_url: &'static str,
    og_image: String,
    current_year: u16,
    
    // Docs content
    title: String,
    description: Option<String>,
    content: String,
    
    // Navigation
    docs_nav: Vec<DocFolder>,
    current_folder: String,
    current_slug: String,
    
    // Breadcrumbs
    breadcrumbs: Vec<BreadcrumbItem>,
}

impl DocsTemplate {
    fn new(folder: &str, slug: &str, title: String, description: Option<String>, content: String, docs_nav: Vec<DocFolder>) -> Self {
        let seo = SeoMeta::default();
        let meta_desc = description.clone().unwrap_or_else(|| format!("{} - Documentation", title));
        
        let canonical_url = if folder.is_empty() {
            format!("{}/docs", seo.base_url)
        } else if slug == "index" {
            format!("{}/docs/{}", seo.base_url, folder)
        } else {
            format!("{}/docs/{}/{}", seo.base_url, folder, slug)
        };
        
        // Build breadcrumbs: Docs > [Folder Display Name] > [Page Title]
        let mut breadcrumbs = vec![
            BreadcrumbItem {
                label: "Docs".to_string(),
                // For root index page, this is self-referential, so use empty URL
                url: if slug == "docs" { String::new() } else { "/docs".to_string() },
            },
        ];
        
        // Add folder breadcrumb if not root
        if !folder.is_empty() {
            // Find folder display name from docs_nav
            let folder_display_name = docs_nav
                .iter()
                .find(|f| f.name == folder)
                .map(|f| f.display_name.clone())
                .unwrap_or_else(|| {
                    // Fallback: convert folder name to display name
                    folder
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
                        .join(" ")
                });
            
            breadcrumbs.push(BreadcrumbItem {
                label: folder_display_name,
                // For folder index page, this is self-referential, so use empty URL
                url: if slug == "index" { String::new() } else { format!("/docs/{}", folder) },
            });
        }
        
        // Add current page title
        breadcrumbs.push(BreadcrumbItem {
            label: title.clone(),
            // Last item is always self-referential, so use empty URL
            url: String::new(),
        });
        
        Self {
            site_name: seo.site_name,
            page_title: title.clone(),
            meta_description: meta_desc,
            meta_keywords: "documentation, AI agents, runpiper",
            canonical_url,
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            title,
            description,
            content,
            docs_nav,
            current_folder: folder.to_string(),
            current_slug: slug.to_string(),
            breadcrumbs,
        }
    }
}

// Blog handler
async fn blog_post(Path(slug): Path<String>) -> Result<BlogTemplate, MarkdownError> {
    let md = get_blog_post(&slug)?;
    
    Ok(BlogTemplate::new(
        &slug,
        md.frontmatter.title,
        md.frontmatter.description,
        md.frontmatter.date,
        md.frontmatter.author,
        md.frontmatter.tags,
        md.html,
    ))
}

// Docs handler
async fn docs_page(Path((folder, slug)): Path<(String, String)>) -> Result<DocsTemplate, MarkdownError> {
    let md = get_doc_page(&folder, &slug)?;
    let docs_nav = list_all_docs()?;
    
    Ok(DocsTemplate::new(
        &folder,
        &slug,
        md.frontmatter.title,
        md.frontmatter.description,
        md.html,
        docs_nav,
    ))
}

// Docs folder index handler (e.g., /docs/getting-started)
async fn docs_folder_index(Path(folder): Path<String>) -> Result<DocsTemplate, MarkdownError> {
    let md = get_doc_index(Some(&folder))?;
    let docs_nav = list_all_docs()?;
    
    Ok(DocsTemplate::new(
        &folder,
        "index",
        md.frontmatter.title,
        md.frontmatter.description,
        md.html,
        docs_nav,
    ))
}

// Docs root index handler (e.g., /docs)
async fn docs_root_index() -> Result<DocsTemplate, MarkdownError> {
    let md = get_doc_index(None)?;
    let docs_nav = list_all_docs()?;
    
    Ok(DocsTemplate::new(
        "",
        "docs",
        md.frontmatter.title,
        md.frontmatter.description,
        md.html,
        docs_nav,
    ))
}

// Blog list template
#[derive(Template)]
#[template(path = "blog-list.html")]
struct BlogListTemplate {
    // SEO fields
    site_name: &'static str,
    page_title: &'static str,
    meta_description: &'static str,
    meta_keywords: &'static str,
    canonical_url: String,
    base_url: &'static str,
    og_image: String,
    current_year: u16,
    
    // Blog list content
    posts: Vec<BlogPostSummary>,
}

impl BlogListTemplate {
    fn new(posts: Vec<BlogPostSummary>) -> Self {
        let seo = SeoMeta::default();
        
        Self {
            site_name: seo.site_name,
            page_title: "Blog",
            meta_description: "Insights, updates, and best practices for building production-grade AI agents with RunPiper. Learn about Rust, performance optimization, and agent deployment strategies.",
            meta_keywords: "AI agents blog, Rust blog, agent runtime, performance, best practices, tutorials",
            canonical_url: format!("{}/blog", seo.base_url),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            posts,
        }
    }
}

// Blog list handler
async fn blog_list() -> Result<BlogListTemplate, MarkdownError> {
    let posts = list_blog_posts()?;
    Ok(BlogListTemplate::new(posts))
}

// 404 template
#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    // SEO fields
    site_name: &'static str,
    page_title: &'static str,
    meta_description: &'static str,
    meta_keywords: &'static str,
    canonical_url: String,
    base_url: &'static str,
    og_image: String,
    current_year: u16,
    
    // Page content
    can_go_back: bool,
}

impl NotFoundTemplate {
    fn new(can_go_back: bool) -> Self {
        let seo = SeoMeta::default();
        
        Self {
            site_name: seo.site_name,
            page_title: "404 - Page Not Found",
            meta_description: "The page you're looking for doesn't exist or has been moved.",
            meta_keywords: "404, page not found",
            canonical_url: format!("{}/404", seo.base_url),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            can_go_back,
        }
    }
}

// 404 handler
async fn not_found_handler(request: Request) -> Response {
    let seo = SeoMeta::default();
    
    // Check if referer is from the same site
    let can_go_back = request
        .headers()
        .get(header::REFERER)
        .and_then(|referer| referer.to_str().ok())
        .map(|referer| {
            // Check if referer is from the same site
            // Check for production domain (runpiper.ai) or localhost for development
            referer.starts_with(seo.base_url)
                || referer.starts_with("http://localhost")
                || referer.starts_with("https://localhost")
                || referer.contains("127.0.0.1")
                // Also check if it contains the same host (for different protocols/ports)
                || (referer.contains("runpiper.ai") && seo.base_url.contains("runpiper.ai"))
        })
        .unwrap_or(false);
    
    let template = NotFoundTemplate::new(can_go_back);
    
    (StatusCode::NOT_FOUND, template).into_response()
}

// Non-async main that runs before tokio
fn main() {
    // Write to file immediately to prove binary is running
    let _ = std::fs::write("/tmp/rust-main-started.txt", "main() called\n");
    
    // Immediate output with explicit flushing
    eprintln!("RUST_APP: ========================================");
    eprintln!("RUST_APP: Starting main function (sync)...");
    eprintln!("RUST_APP: ========================================");
    std::io::stderr().flush().ok();
    
    // Now start the async runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_main());
}

async fn async_main() {
    eprintln!("RUST_APP: In async_main()");
    std::io::stderr().flush().ok();
    
    // Set up panic hook to capture panics
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("RUST_APP: PANIC occurred!");
        eprintln!("RUST_APP: Location: {:?}", panic_info.location());
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("RUST_APP: Message: {}", s);
        }
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("RUST_APP: Message: {}", s);
        }
        std::io::stderr().flush().ok();
    }));
    
    print!("RUST_APP: About to call run()...\n");
    std::io::stdout().flush().ok();
    
    // Better error handling
    if let Err(e) = run().await {
        eprintln!("RUST_APP: Error starting server: {}", e);
        std::io::stderr().flush().ok();
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("RUST_APP: In run() function");
    std::io::stderr().flush().ok();
    
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    eprintln!("RUST_APP: PORT = {}", port);
    std::io::stderr().flush().ok();
    
    // Build middleware stack
    let middleware = ServiceBuilder::new()
        // Add compression (gzip, brotli, zstd)
        .layer(CompressionLayer::new())
        // Security headers
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ));
    
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/blog", get(blog_list))
        .route("/blog/:slug", get(blog_post))
        .route("/docs", get(docs_root_index))
        .route("/docs/:folder", get(docs_folder_index))
        .route("/docs/:folder/:slug", get(docs_page))
        // Serve static files with aggressive caching
        .nest_service("/static", ServeDir::new("static"))
        // 404 handler for all unmatched routes
        .fallback(not_found_handler)
        .layer(middleware);
    
    let addr = format!("0.0.0.0:{}", port);
    
    eprintln!("RUST_APP: Starting server on {}", addr);
    std::io::stderr().flush().ok();
    
    eprintln!("RUST_APP: Attempting to bind to {}", addr);
    std::io::stderr().flush().ok();
    
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => {
            eprintln!("RUST_APP: Successfully bound to {}", addr);
            std::io::stderr().flush().ok();
            l
        }
        Err(e) => {
            eprintln!("RUST_APP: ERROR binding to {}: {}", addr, e);
            std::io::stderr().flush().ok();
            return Err(Box::new(e));
        }
    };
    
    eprintln!("RUST_APP: Server bound successfully, listening on http://{}", addr);
    std::io::stderr().flush().ok();
    
    eprintln!("RUST_APP: About to start serving...");
    std::io::stderr().flush().ok();
    
    eprintln!("RUST_APP: Calling axum::serve()...");
    std::io::stderr().flush().ok();
    
    match axum::serve(listener, app).await {
        Ok(_) => {
            eprintln!("RUST_APP: axum::serve() returned successfully");
            std::io::stderr().flush().ok();
            Ok(())
        }
        Err(e) => {
            eprintln!("RUST_APP: ERROR in axum::serve(): {}", e);
            std::io::stderr().flush().ok();
            Err(Box::new(e))
        }
    }
}
