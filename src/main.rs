use askama::Template;
use axum::{routing::get, Router, Json};
use serde::Serialize;

mod seo;
use seo::SeoMeta;

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
            meta_description: "Welcome to our website. Discover our products and services.",
            meta_keywords: "website, services, products",
            canonical_url: seo.base_url.to_string(),
            base_url: seo.base_url,
            og_image: format!("{}/og-image.png", seo.base_url),
            current_year: seo.current_year,
            
            heading: "Hello, World!",
            subheading: "Welcome to your new Axum-powered website",
            welcome_text: "This is a fully SEO-optimized website scaffold built with Axum and Askama templates. Every page includes proper meta tags, Open Graph data, Twitter cards, and structured data for maximum search engine visibility.",
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

// Non-async main that runs before tokio
fn main() {
    // Write to file immediately to prove binary is running
    let _ = std::fs::write("/tmp/rust-main-started.txt", "main() called\n");
    
    // Immediate output with explicit flushing
    use std::io::Write;
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
    use std::io::Write;
    
    eprintln!("RUST_APP: In run() function");
    std::io::stderr().flush().ok();
    
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    eprintln!("RUST_APP: PORT = {}", port);
    std::io::stderr().flush().ok();
    
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health));
    let addr = format!("0.0.0.0:{}", port);
    
    eprintln!("RUST_APP: Starting server on {}", addr);
    std::io::stderr().flush().ok();
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    eprintln!("RUST_APP: Server bound successfully, listening on http://{}", addr);
    std::io::stderr().flush().ok();
    
    eprintln!("RUST_APP: About to start serving...");
    std::io::stderr().flush().ok();
    
    axum::serve(listener, app).await?;
    Ok(())
}
