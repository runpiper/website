use askama::Template;
use axum::{
    routing::get,
    Router,
    Json,
    extract::Path,
};
use serde::Serialize;
use std::io::Write;
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
};
use tower::ServiceBuilder;
use axum::http::{header, HeaderValue};

mod seo;
use seo::SeoMeta;

mod markdown;
use markdown::{get_blog_post, get_doc_page, MarkdownError};

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
}

impl BlogTemplate {
    fn new(slug: &str, title: String, description: Option<String>, date: Option<String>, author: Option<String>, tags: Option<Vec<String>>, content: String) -> Self {
        let seo = SeoMeta::default();
        let meta_desc = description.unwrap_or_else(|| format!("{} - {}", title, seo.site_name));
        
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
}

impl DocsTemplate {
    fn new(folder: &str, slug: &str, title: String, description: Option<String>, content: String) -> Self {
        let seo = SeoMeta::default();
        let meta_desc = description.clone().unwrap_or_else(|| format!("{} - Documentation", title));
        
        Self {
            site_name: seo.site_name,
            page_title: title.clone(),
            meta_description: meta_desc,
            meta_keywords: "documentation, AI agents, runpiper",
            canonical_url: format!("{}/docs/{}/{}", seo.base_url, folder, slug),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            title,
            description,
            content,
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
    
    Ok(DocsTemplate::new(
        &folder,
        &slug,
        md.frontmatter.title,
        md.frontmatter.description,
        md.html,
    ))
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
        .route("/blog/:slug", get(blog_post))
        .route("/docs/:folder/:slug", get(docs_page))
        // Serve static files with aggressive caching
        .nest_service("/static", ServeDir::new("static"))
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
