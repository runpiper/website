# Performance Checklist for Hyper-Fast Static Pages

## ✅ Already Implemented

### Core Infrastructure

-   [x] **Compile-time templates** (Askama) - Zero runtime template overhead
-   [x] **HTTP compression** (gzip, brotli, zstd) via tower-http
-   [x] **Static file serving** with automatic compression
-   [x] **Security headers** (X-Content-Type-Options, X-Frame-Options, HSTS)
-   [x] **System fonts** - No web font loading delays
-   [x] **Release optimizations** - LTO, strip symbols, opt-level 3
-   [x] **Multi-stage Docker build** - Small, optimized images
-   [x] **SEO meta tags** - OG, Twitter, Schema.org
-   [x] **Semantic HTML** with accessibility features

## 🔲 To Consider When Adding Content

### CSS Strategy

-   [ ] Inline critical CSS in `<head>` for above-the-fold content
-   [ ] Defer non-critical CSS or use `media="print" onload="this.media='all'"`
-   [ ] Minify CSS before deploying
-   [ ] Consider CSS modules or utility-first approach for minimal CSS
-   [ ] Use CSS containment (`contain: layout style paint`) where appropriate

### Image Optimization

-   [ ] Use WebP/AVIF with JPEG/PNG fallbacks
-   [ ] Implement `<picture>` element for responsive images
-   [ ] Add proper `width` and `height` attributes to prevent CLS
-   [ ] Use `loading="lazy"` for below-the-fold images
-   [ ] Consider blur-up placeholders for hero images
-   [ ] Optimize images with tools like `oxipng`, `cjpeg-mozjpeg`, or `webp`

### JavaScript Strategy

-   [ ] Prefer vanilla JS or lightweight libraries
-   [ ] Use `defer` or `async` for non-critical scripts
-   [ ] Code-split if using bundlers
-   [ ] Minify and tree-shake in production
-   [ ] Consider ES modules with `type="module"`

### Resource Hints

Add to `<head>` as needed:

-   [ ] `<link rel="preload">` for critical assets (fonts, hero images)
-   [ ] `<link rel="dns-prefetch">` for external domains
-   [ ] `<link rel="preconnect">` for critical third-party domains
-   [ ] `<link rel="modulepreload">` for ES module dependencies

### Fonts (if you add web fonts)

-   [ ] Use `font-display: swap` or `optional`
-   [ ] Subset fonts to only needed characters
-   [ ] Use WOFF2 format
-   [ ] Self-host fonts instead of Google Fonts
-   [ ] Preload critical font files

### Content Strategy

-   [ ] Keep HTML minimal and semantic
-   [ ] Avoid unnecessary DOM depth
-   [ ] Use semantic elements (`<header>`, `<nav>`, `<main>`, `<article>`)
-   [ ] Minimize third-party scripts
-   [ ] Lazy-load comments, social widgets, etc.

### Caching Strategy

-   [ ] Set appropriate `Cache-Control` headers for static assets
    -   Images, CSS, JS: `max-age=31536000, immutable` (with versioning)
    -   HTML: `max-age=300` or `no-cache` (validate with CDN)
-   [ ] Use ETags for validation
-   [ ] Consider service worker for offline support

### Monitoring & Validation

-   [ ] Test with Lighthouse (aim for 100 performance score)
-   [ ] Validate with PageSpeed Insights
-   [ ] Check Core Web Vitals:
    -   LCP (Largest Contentful Paint) < 2.5s
    -   FID (First Input Delay) < 100ms
    -   CLS (Cumulative Layout Shift) < 0.1
-   [ ] Test on slow 3G connections
-   [ ] Verify compression is working (`Accept-Encoding` / `Content-Encoding`)

## 🚀 Advanced Optimizations (Optional)

### HTTP/2 & HTTP/3

-   [ ] Ensure hosting supports HTTP/2 (multiplexing, header compression)
-   [ ] Consider HTTP/3/QUIC if supported by host

### CDN Integration

-   [ ] Distribute static assets via CDN
-   [ ] Use geo-distributed edge locations
-   [ ] Implement edge caching rules

### Build-Time Optimization

-   [ ] Generate critical CSS automatically
-   [ ] Inline small images as data URIs or SVG
-   [ ] Pre-compress assets (gzip, brotli) at build time
-   [ ] Generate multiple image formats/sizes automatically

### Database/API (When Needed)

-   [ ] Add database connection pooling
-   [ ] Implement request caching (Redis/Memcached)
-   [ ] Use streaming responses for large data
-   [ ] Implement proper rate limiting

## 📊 Performance Targets

| Metric                   | Target  | Excellent |
| ------------------------ | ------- | --------- |
| First Byte (TTFB)        | < 200ms | < 100ms   |
| First Contentful Paint   | < 1.0s  | < 0.5s    |
| Largest Contentful Paint | < 2.5s  | < 1.5s    |
| Time to Interactive      | < 3.0s  | < 2.0s    |
| Total Blocking Time      | < 300ms | < 100ms   |
| Cumulative Layout Shift  | < 0.1   | < 0.05    |
| HTML Size                | < 50KB  | < 20KB    |
| Total Page Weight        | < 500KB | < 200KB   |

## 🛠️ Recommended Tools

### Development

-   `cargo clippy` - Rust linting
-   `cargo bench` - Benchmarking
-   Chrome DevTools - Network and Performance panels
-   Lighthouse CI - Automated performance testing

### Image Optimization

-   `oxipng` - Optimize PNG files
-   `cjpeg-mozjpeg` - Optimize JPEG files
-   `cwebp` / `avifenc` - Convert to modern formats
-   `imagemagick` - Batch processing and resizing

### Validation

-   WebPageTest.org - Detailed performance analysis
-   PageSpeed Insights - Google's performance scoring
-   Chrome Lighthouse - In-browser auditing
-   GTmetrix - Performance and optimization recommendations

## 📝 Notes

-   This setup is designed for **server-rendered static pages**
-   Compression happens automatically at runtime
-   No build tools needed for basic pages (pure HTML/CSS)
-   All static assets in `/static/` are served with compression
-   Security headers are applied globally
