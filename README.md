# Runpiper Public Website

A modern, responsive, and performant website built with compile-time templates and a comprehensive responsive design system.

## Features

- 🎨 **Beautiful Typography**: IBM Plex Serif for headings, IBM Plex Sans for body text
- 📱 **Fully Responsive**: Mobile-first design with clear breakpoints for all devices
- ⚡ **High Performance**: Compile-time templates, automatic compression (gzip/brotli/zstd)
- 🔍 **SEO Optimized**: Complete meta tags, Open Graph, Twitter Cards, Schema.org structured data
- ♿ **Accessible**: WCAG compliant, semantic HTML, keyboard navigation
- 🎯 **Zero Dependencies**: No JavaScript frameworks, pure CSS responsive system

## Responsive Design System

The website uses a comprehensive responsive breakpoint system with four device categories:

- **Phone**: < 768px - Mobile devices in portrait/landscape
- **Tablet**: 768px - 1023px - iPads, Android tablets, small laptops
- **Desktop**: 1024px - 1439px - Standard desktops and laptops
- **Large Desktop**: 1440px+ - Large monitors and 4K displays

### Key Features

- Mobile-first approach with progressive enhancement
- Automatic typography scaling per breakpoint
- Responsive spacing system with CSS variables
- Flexible grid systems (card grid + row/column)
- Comprehensive utility classes for responsive layouts
- Container system with automatic max-widths

### Documentation

- **Full Guide**: [`/docs/RESPONSIVE.md`](docs/RESPONSIVE.md) - Complete documentation with examples
- **Quick Reference**: [`/docs/RESPONSIVE-QUICK-REFERENCE.md`](docs/RESPONSIVE-QUICK-REFERENCE.md) - Cheat sheet
- **Implementation Summary**: [`/docs/RESPONSIVE-IMPLEMENTATION-SUMMARY.md`](docs/RESPONSIVE-IMPLEMENTATION-SUMMARY.md) - Overview of changes
- **Demo Page**: `/templates/responsive-demo.html` - Interactive demonstration

## Project Structure

```
website/
├── templates/
│   ├── base.html              # Base template with meta tags
│   ├── home.html              # Home page
│   └── responsive-demo.html   # Responsive system demo
├── static/
│   ├── css/
│   │   ├── base.css          # Foundation & reset
│   │   ├── fonts.css         # Font declarations
│   │   ├── typography.css    # Text styles
│   │   ├── responsive.css    # Responsive breakpoints
│   │   ├── main.css          # Components & layout
│   │   └── home.css          # Home page styles
│   └── fonts/
│       └── IBM Plex fonts    # Self-hosted fonts
└── docs/
    ├── RESPONSIVE.md
    ├── RESPONSIVE-QUICK-REFERENCE.md
    ├── RESPONSIVE-IMPLEMENTATION-SUMMARY.md
    └── FONTS.md

## Quick Start

### Using the Container System

```html
<section class="section">
    <div class="container">
        <h2>Section Title</h2>
        <p>Your content here...</p>
    </div>
</section>
```

### Responsive Card Grid

```html
<div class="card-grid">
    <div class="card">
        <h3>Card Title</h3>
        <p>Card content</p>
    </div>
    <!-- More cards... -->
</div>
```

### Two-Column Layout

```html
<div class="row">
    <div class="col col-phone-12 col-tablet-6">
        Left column
    </div>
    <div class="col col-phone-12 col-tablet-6">
        Right column
    </div>
</div>
```

## Testing

Test the responsive system at these breakpoints:

- **320px** - Minimum (iPhone SE)
- **375px** - Standard phone
- **768px** - Tablet portrait
- **1024px** - Desktop
- **1440px** - Large desktop
- **1920px** - Full HD

Use browser DevTools responsive mode (F12 → Device Toolbar) for testing.

## Performance

- **CSS Total**: ~21KB uncompressed, ~6-7KB compressed
- **Fonts**: WOFF2 format, preloaded
- **No JavaScript**: Pure CSS responsive system
- **HTTP/2**: Optimized multiplexing
- **Automatic Compression**: gzip/brotli/zstd

## Browser Support

- Chrome/Edge (latest 2 versions)
- Firefox (latest 2 versions)
- Safari (latest 2 versions)
- Mobile Safari (iOS 12+)
- Chrome Mobile (Android 8+)

## Development

### Adding New Pages

1. Create template in `/templates/`
2. Create page-specific CSS in `/static/css/`
3. Use responsive containers and utilities
4. Test at all breakpoints

### CSS Architecture

Stylesheets load in order:
1. `base.css` - Foundation
2. `fonts.css` - Fonts
3. `typography.css` - Text
4. `responsive.css` - Breakpoints
5. `main.css` - Components
6. Page-specific CSS

See [`/static/css/README.md`](static/css/README.md) for details.

## Resources

- [Typography Guide](docs/FONTS.md)
- [CSS Organization](static/css/README.md)
- [Responsive System](docs/RESPONSIVE.md)
- [Quick Reference](docs/RESPONSIVE-QUICK-REFERENCE.md)

## License

All rights reserved.
