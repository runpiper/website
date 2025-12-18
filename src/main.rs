use askama::Template;
use axum::{routing::get, Router, Json};
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
            
            heading: "Indestructible Agent Runtime",
            subheading: "Built in Rust for performance, stability and security.",
            welcome_text: "RunPiper is the open-source Supabase for AI agents. Built in Rust for rock-solid performance and stability, it provides long-running agent instances that are completely decoupled from your codebase. Focus on building great user experiences and getting actual work done—let RunPiper handle the complexity of managing your agents.",
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
