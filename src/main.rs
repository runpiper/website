use askama::Template;
use axum::{routing::get, Router};

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

#[tokio::main]
async fn main() {
    // Better error handling
    if let Err(e) = run().await {
        eprintln!("Error starting server: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(home));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
