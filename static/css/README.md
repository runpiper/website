# CSS Organization

All styles are organized into separate CSS files. No inline styles in HTML templates.

## File Structure

### Core Styles (Loaded on Every Page)

**`base.css`** - Reset and fundamental styles

-   CSS reset (box-sizing, margin, padding)
-   HTML and body defaults
-   Base typography setup
-   Link and image defaults
-   Skip-link accessibility styles

**`fonts.css`** - Font-face declarations

-   @font-face rules for IBM Plex Sans
-   @font-face rules for IBM Plex Serif
-   All font weights (400, 500, 600, 700)

**`typography.css`** - Typography system

-   Heading styles (h1-h6)
-   Paragraph styles
-   Text utilities (lead, small, bold, italic)
-   Blockquotes and code
-   Lists
-   Font family utilities

**`responsive.css`** - Responsive breakpoint system

-   Mobile-first breakpoints (phone, tablet, desktop, desktop-lg)
-   Container responsive styles
-   Typography scaling per breakpoint
-   Responsive spacing system
-   Grid system adjustments
-   Display and layout utilities
-   See `/docs/RESPONSIVE.md` for full documentation

**`main.css`** - Layout and component styles

-   Header and navigation
-   Hero sections
-   Cards and grids
-   Buttons
-   Footer
-   Component-specific styles

### Page-Specific Styles

**`home.css`** - Home page specific styles

-   Hero section overrides
-   Features section
-   Feature grid and cards
-   Welcome section

**`example.css`** - Example CSS patterns

-   Modern CSS techniques
-   Animation examples
-   Grid and layout examples

## Load Order

In `base.html`:

```html
1. base.css - Foundation 2. fonts.css - Font loading 3. typography.css - Text
styles 4. responsive.css - Responsive breakpoints 5. main.css - Components
```

In page templates (via `{% block head %}`):

```html
6. home.css - Page-specific (home page) 7. [other].css - Page-specific (other
pages)
```

## Best Practices

### ✅ Do

-   Keep page-specific styles in separate files
-   Use semantic class names
-   Leverage CSS custom properties for theming
-   Use modern CSS (grid, flexbox, clamp)
-   Keep specificity low

### ❌ Don't

-   Add inline styles in HTML
-   Use !important (unless absolutely necessary)
-   Create overly specific selectors
-   Duplicate styles across files
-   Use presentational class names

## Adding New Pages

When creating a new page:

1. Create `/static/css/pagename.css`
2. Add page-specific styles there
3. Link in template: `{% block head %}<link rel="stylesheet" href="/static/css/pagename.css" />{% endblock %}`

Example:

```html
{% extends "base.html" %} {% block head %}
<link rel="stylesheet" href="/static/css/about.css" />
{% endblock %} {% block content %}
<!-- Your content -->
{% endblock %}
```

## Performance

All CSS files are:

-   ✅ Automatically compressed (gzip/brotli/zstd)
-   ✅ Served from same domain (HTTP/2 multiplexing)
-   ✅ Cached by browser
-   ✅ Minifiable for production

### Production Optimization

For production, consider:

1. Minifying CSS files
2. Combining critical CSS
3. Adding cache-busting with versioning
4. Using PurgeCSS to remove unused styles

## File Sizes (Uncompressed)

-   base.css: ~1.5KB
-   fonts.css: ~1.5KB
-   typography.css: ~4KB
-   responsive.css: ~8KB
-   main.css: ~4KB
-   home.css: ~2KB

**Total:** ~21KB uncompressed, ~6-7KB compressed with brotli

## CSS Custom Properties

Define reusable values in `:root`:

```css
:root {
    --color-primary: #007bff;
    --color-text: #1a1a1a;
    --color-bg: #fafafa;
    --spacing-unit: 1rem;
    --font-sans: "IBM Plex Sans", system-ui, sans-serif;
    --font-serif: "IBM Plex Serif", Georgia, serif;
}
```

Use throughout your CSS:

```css
body {
    font-family: var(--font-sans);
    color: var(--color-text);
}
```

### Responsive Spacing Variables

The responsive system provides automatic spacing scaling:

```css
.my-component {
    padding: var(--spacing-xl); /* Scales: 2rem → 3rem → 3rem → 4rem */
    margin-bottom: var(--spacing-2xl); /* Scales: 3rem → 4rem → 5rem → 6rem */
}
```

See `/docs/RESPONSIVE.md` for complete breakpoint system documentation.

## Maintenance

-   Review and consolidate duplicate styles quarterly
-   Remove unused styles when pages are removed
-   Keep CSS files organized by concern (layout, components, utilities)
-   Document complex or non-obvious styles with comments
