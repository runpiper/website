# IBM Plex Fonts Setup

## Current Implementation

We're using **IBM Plex Sans** for body text and **IBM Plex Serif** for headings.

### Current Setup (Google Fonts)
- ✅ Easy to implement
- ✅ Cached across sites
- ✅ Includes `font-display: swap` for better performance
- ⚠️ Requires external connection to Google

Font weights loaded:
- **IBM Plex Sans**: 400 (regular), 500 (medium), 600 (semi-bold)
- **IBM Plex Serif**: 500 (medium), 600 (semi-bold), 700 (bold)

## 🚀 Self-Hosting (Recommended for Production)

Self-hosting provides:
- **Better performance** (one less DNS lookup, no external connection)
- **Full control** over caching headers
- **Privacy** (no tracking from Google)
- **Reliability** (no external dependency)

### How to Self-Host

#### 1. Download IBM Plex Fonts

```bash
# Clone the IBM Plex repository
cd /tmp
git clone https://github.com/IBM/plex.git

# Copy WOFF2 files to your project
cd plex
cp IBM-Plex-Sans/fonts/complete/woff2/IBMPlexSans-Regular.woff2 \
   IBM-Plex-Sans/fonts/complete/woff2/IBMPlexSans-Medium.woff2 \
   IBM-Plex-Sans/fonts/complete/woff2/IBMPlexSans-SemiBold.woff2 \
   /path/to/your/project/static/fonts/

cp IBM-Plex-Serif/fonts/complete/woff2/IBMPlexSerif-Medium.woff2 \
   IBM-Plex-Serif/fonts/complete/woff2/IBMPlexSerif-SemiBold.woff2 \
   IBM-Plex-Serif/fonts/complete/woff2/IBMPlexSerif-Bold.woff2 \
   /path/to/your/project/static/fonts/
```

#### 2. Create Font Face Declarations

Create `/static/css/fonts.css`:

```css
/* IBM Plex Sans */
@font-face {
    font-family: 'IBM Plex Sans';
    src: url('/static/fonts/IBMPlexSans-Regular.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Sans';
    src: url('/static/fonts/IBMPlexSans-Medium.woff2') format('woff2');
    font-weight: 500;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Sans';
    src: url('/static/fonts/IBMPlexSans-SemiBold.woff2') format('woff2');
    font-weight: 600;
    font-style: normal;
    font-display: swap;
}

/* IBM Plex Serif */
@font-face {
    font-family: 'IBM Plex Serif';
    src: url('/static/fonts/IBMPlexSerif-Medium.woff2') format('woff2');
    font-weight: 500;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Serif';
    src: url('/static/fonts/IBMPlexSerif-SemiBold.woff2') format('woff2');
    font-weight: 600;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Serif';
    src: url('/static/fonts/IBMPlexSerif-Bold.woff2') format('woff2');
    font-weight: 700;
    font-style: normal;
    font-display: swap;
}
```

#### 3. Update `base.html`

Replace the Google Fonts link with:

```html
<!-- Self-hosted IBM Plex fonts -->
<link rel="preload" href="/static/fonts/IBMPlexSans-Regular.woff2" as="font" type="font/woff2" crossorigin />
<link rel="preload" href="/static/fonts/IBMPlexSerif-SemiBold.woff2" as="font" type="font/woff2" crossorigin />
<link rel="stylesheet" href="/static/css/fonts.css" />
```

#### 4. Update Dockerfile

The fonts will automatically be included when you copy the `/static/` directory.

## 📊 Performance Comparison

| Method | DNS Lookup | Connections | File Size | Caching |
|--------|------------|-------------|-----------|---------|
| Google Fonts | 2 domains | 2 | Optimized | Shared cache |
| Self-hosted | 0 extra | 0 extra | ~120KB total | Your control |

### Self-hosting wins because:
1. **No external DNS lookup** (~20-50ms saved)
2. **Same origin** (can use HTTP/2 multiplexing)
3. **Aggressive caching** (set `Cache-Control: max-age=31536000, immutable`)
4. **WOFF2 compression** is excellent (~30% smaller than TTF/OTF)

## 🎨 Font Usage Guidelines

### IBM Plex Sans (Body)
Use for:
- Body text
- UI elements (buttons, forms)
- Navigation
- Captions and labels

Weights:
- 400: Regular body text
- 500: Emphasized text, UI elements
- 600: Strong emphasis, buttons

### IBM Plex Serif (Headings)
Use for:
- All headings (h1-h6)
- Pull quotes
- Feature callouts
- Hero text

Weights:
- 500: h4-h6, subheadings
- 600: h2-h3, standard headings
- 700: h1, hero titles

## 💡 Best Practices

### 1. Preload Critical Fonts
Only preload fonts used above the fold:
```html
<link rel="preload" href="/static/fonts/IBMPlexSans-Regular.woff2" as="font" type="font/woff2" crossorigin />
<link rel="preload" href="/static/fonts/IBMPlexSerif-SemiBold.woff2" as="font" type="font/woff2" crossorigin />
```

### 2. Use font-display: swap
Already included in the `@font-face` declarations. Shows fallback font immediately, swaps when custom font loads.

### 3. Subset Fonts (Advanced)
For even better performance, create subsets with only the characters you need:

```bash
# Using pyftsubset (from fonttools)
pip install fonttools brotli

pyftsubset IBMPlexSans-Regular.woff2 \
  --output-file=IBMPlexSans-Regular-subset.woff2 \
  --flavor=woff2 \
  --layout-features="*" \
  --unicodes="U+0020-007F,U+00A0-00FF"  # Basic Latin + Latin-1
```

### 4. Cache Headers
If self-hosting, add cache headers in `main.rs`:

```rust
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{header, HeaderValue};

// For font files specifically
.layer(SetResponseHeaderLayer::overriding(
    header::CACHE_CONTROL,
    HeaderValue::from_static("public, max-age=31536000, immutable"),
))
```

## 🔍 License

IBM Plex is open source under the [SIL Open Font License 1.1](https://github.com/IBM/plex/blob/master/LICENSE.txt).

- ✅ Free for commercial use
- ✅ Can be bundled with software
- ✅ Can be modified
- ✅ Must include license file

## 📚 Additional Resources

- [IBM Plex GitHub](https://github.com/IBM/plex)
- [IBM Plex Website](https://www.ibm.com/plex/)
- [Google Fonts: IBM Plex](https://fonts.google.com/specimen/IBM+Plex+Sans)
- [Font subsetting guide](https://markoskon.com/creating-font-subsets/)

## ⚡ Current Performance Impact

With Google Fonts (current):
- ~15-20KB transferred (with compression)
- ~50-100ms extra for DNS + connection
- FOUT (Flash of Unstyled Text) minimal due to `font-display: swap`

Expected with self-hosting:
- ~120KB initial (cached forever)
- No external connections
- Parallel download with HTTP/2
- **~50-100ms faster** on subsequent visits

