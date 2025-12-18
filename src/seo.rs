use std::time::{SystemTime, UNIX_EPOCH};

/// SEO metadata configuration for the site
pub struct SeoMeta {
    pub site_name: &'static str,
    pub base_url: &'static str,
    pub current_year: u16,
}

impl Default for SeoMeta {
    fn default() -> Self {
        // Calculate current year
        let current_year = {
            let duration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let secs = duration.as_secs();
            // Approximate year calculation
            1970 + (secs / 31_557_600) as u16
        };

        Self {
            site_name: "My Website",
            base_url: "https://example.com",
            current_year,
        }
    }
}

/// Helper to generate page-specific SEO data
pub struct PageSeo {
    pub title: String,
    pub description: String,
    pub keywords: String,
    pub canonical_path: String,
    pub og_type: String,
}

impl PageSeo {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            keywords: String::new(),
            canonical_path: String::new(),
            og_type: "website".to_string(),
        }
    }

    pub fn with_keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = keywords.into();
        self
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.canonical_path = path.into();
        self
    }

    pub fn as_article(mut self) -> Self {
        self.og_type = "article".to_string();
        self
    }
}

