# Responsive Design System

This document outlines the responsive breakpoint system used throughout the website.

## Table of Contents

- [Overview](#overview)
- [Breakpoints](#breakpoints)
- [Container System](#container-system)
- [Typography Scaling](#typography-scaling)
- [Spacing System](#spacing-system)
- [Grid System](#grid-system)
- [Utility Classes](#utility-classes)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Overview

The website uses a **mobile-first responsive design approach**, meaning:

1. Base styles are written for mobile devices (smallest screens)
2. Styles are progressively enhanced for larger screens using `@media` queries
3. Content is accessible and usable at any screen size

All responsive styles are centralized in `static/css/responsive.css`.

## Breakpoints

The system defines four clear device categories:

| Device Category | Breakpoint Range | CSS Media Query | Target Devices |
|----------------|------------------|-----------------|----------------|
| **Phone (Mobile)** | < 768px | Base styles (no query) | Smartphones in portrait/landscape |
| **Tablet** | 768px - 1023px | `@media (min-width: 768px)` | iPads, Android tablets, small laptops |
| **Desktop (Regular)** | 1024px - 1439px | `@media (min-width: 1024px)` | Standard laptops, desktops |
| **Desktop (Large)** | 1440px+ | `@media (min-width: 1440px)` | Large monitors, 4K displays |

### CSS Variables

Breakpoint values are available as CSS custom properties:

```css
:root {
    --breakpoint-phone: 767px;
    --breakpoint-tablet: 768px;
    --breakpoint-desktop: 1024px;
    --breakpoint-desktop-lg: 1440px;
}
```

## Container System

Containers provide consistent content width and padding across breakpoints.

### Container Max-Widths

| Breakpoint | Container Max-Width | Container Narrow Max-Width |
|-----------|-------------------|---------------------------|
| Phone | 100% | 100% |
| Tablet | 720px | 640px |
| Desktop | 960px | 720px |
| Desktop Large | 1200px | 800px |

### Usage

```html
<!-- Standard container -->
<div class="container">
    <h1>Page Title</h1>
    <p>Content goes here...</p>
</div>

<!-- Narrow container (for reading content) -->
<div class="container-narrow">
    <article>
        <h1>Blog Post Title</h1>
        <p>Long-form content is easier to read in narrower containers...</p>
    </article>
</div>
```

### Container Padding

Container horizontal padding automatically adjusts:

- **Phone**: 1rem (16px)
- **Tablet**: 2rem (32px)
- **Desktop**: 3rem (48px)
- **Desktop Large**: 4rem (64px)

## Typography Scaling

Font sizes scale progressively to ensure readability on all devices.

### Heading Sizes

| Element | Phone | Tablet | Desktop | Desktop Large |
|---------|-------|--------|---------|---------------|
| `<h1>` | 28px / 32px | 32px / 36px | 36px / 40px | 48px / 52px |
| `<h2>` | 24px / 28px | 28px / 32px | 32px / 36px | 36px / 40px |
| `<h3>` | 20px / 24px | 24px / 28px | 28px / 32px | 28px / 32px |

*Format: font-size / line-height*

### Body Text

| Breakpoint | Base Font Size | Lead Text |
|-----------|----------------|-----------|
| Phone | 16px | 1.125rem (18px) |
| Tablet | 16px | 1.25rem (20px) |
| Desktop | 18px | 1.25rem (22.5px) |
| Desktop Large | 18px | 1.375rem (24.75px) |

### Usage

```html
<!-- Headings automatically scale -->
<h1>This heading scales responsively</h1>
<h2>So does this subheading</h2>

<!-- Lead paragraph for intro text -->
<p class="lead">
    This is an introductory paragraph with larger text.
</p>

<!-- Regular paragraph -->
<p>
    This is regular body text that remains readable at all screen sizes.
</p>
```

## Spacing System

The spacing scale uses CSS custom properties that adjust per breakpoint.

### Spacing Scale

| Variable | Phone | Tablet | Desktop | Desktop Large |
|----------|-------|--------|---------|---------------|
| `--spacing-xs` | 0.25rem | 0.25rem | 0.25rem | 0.25rem |
| `--spacing-sm` | 0.5rem | 0.5rem | 0.5rem | 0.5rem |
| `--spacing-md` | 1rem | 1rem | 1rem | 1rem |
| `--spacing-lg` | 1.5rem | 2rem | 2rem | 2.5rem |
| `--spacing-xl` | 2rem | 3rem | 3rem | 4rem |
| `--spacing-2xl` | 3rem | 4rem | 5rem | 6rem |
| `--spacing-3xl` | 4rem | 5rem | 6rem | 8rem |

### Section Padding

Common sections adjust their padding automatically:

| Section | Phone | Tablet | Desktop | Desktop Large |
|---------|-------|--------|---------|---------------|
| `.section` | 2rem | 4rem | 6rem | 8rem |
| `.hero` | 2rem | 4rem | 6rem | 8rem |
| `<header>` | 1.5rem | 3rem | 3rem | 4rem |
| `<footer>` | 2rem | 4rem | 5rem | 8rem |

### Usage

```css
/* Use spacing variables in your custom CSS */
.my-component {
    padding: var(--spacing-xl);
    margin-bottom: var(--spacing-lg);
}
```

```html
<!-- Sections automatically get responsive padding -->
<section class="section">
    <div class="container">
        <h2>Section Title</h2>
        <p>Content...</p>
    </div>
</section>
```

## Grid System

The card grid system automatically adjusts columns based on screen size.

### Card Grid Behavior

| Breakpoint | Columns | Gap |
|-----------|---------|-----|
| Phone | 1 column | 1.5rem |
| Tablet | Auto-fit (min 300px) | 3rem |
| Desktop | Auto-fit (min 320px) | 3rem |
| Desktop Large | Auto-fit (min 350px) | 6rem |

### Usage

```html
<div class="card-grid">
    <div class="card">
        <h3>Card Title 1</h3>
        <p>Card content...</p>
    </div>
    <div class="card">
        <h3>Card Title 2</h3>
        <p>Card content...</p>
    </div>
    <div class="card">
        <h3>Card Title 3</h3>
        <p>Card content...</p>
    </div>
</div>
```

### Flexible Row/Column System

For custom layouts, use the row/column system:

```html
<div class="row">
    <div class="col col-phone-12 col-tablet-6 col-desktop-4">
        Column 1
    </div>
    <div class="col col-phone-12 col-tablet-6 col-desktop-4">
        Column 2
    </div>
    <div class="col col-phone-12 col-tablet-12 col-desktop-4">
        Column 3
    </div>
</div>
```

Column classes:
- `col-phone-{12|6}` - Phone layouts
- `col-tablet-{12|6|4|3}` - Tablet layouts
- `col-desktop-{12|6|4|3}` - Desktop layouts

## Utility Classes

### Display Utilities

Hide/show elements at specific breakpoints:

```html
<!-- Hide on phones only -->
<div class="hide-phone">
    This content is hidden on phones
</div>

<!-- Hide on tablets only -->
<div class="hide-tablet">
    This content is hidden on tablets
</div>

<!-- Hide on desktops only -->
<div class="hide-desktop">
    This content is hidden on regular desktops
</div>

<!-- Show only on phones -->
<div class="show-phone-only">
    Phone-specific content
</div>

<!-- Show only on tablets -->
<div class="show-tablet-only">
    Tablet-specific content
</div>

<!-- Show only on desktop -->
<div class="show-desktop-only">
    Desktop-specific content
</div>
```

### Text Alignment

Responsive text alignment:

```html
<!-- Center text on phones, left-align on larger screens -->
<p class="text-center-phone">
    This text is centered on mobile
</p>

<!-- Center text only on tablets -->
<p class="text-center-tablet">
    This text is centered on tablets
</p>

<!-- Center text only on desktop -->
<p class="text-center-desktop">
    This text is centered on desktop
</p>
```

### Responsive Spacing

Apply responsive padding/margin that scales automatically:

```html
<div class="p-responsive">
    Responsive padding on all sides
</div>

<div class="px-responsive">
    Responsive horizontal padding
</div>

<div class="py-responsive">
    Responsive vertical padding
</div>

<div class="m-responsive">
    Responsive margin on all sides
</div>

<div class="mx-responsive">
    Responsive horizontal margin
</div>

<div class="my-responsive">
    Responsive vertical margin
</div>
```

## Best Practices

### 1. Mobile-First Development

Always design and develop for mobile first, then enhance for larger screens:

```css
/* ✓ Good - Mobile first */
.component {
    flex-direction: column;
}

@media (min-width: 768px) {
    .component {
        flex-direction: row;
    }
}

/* ✗ Bad - Desktop first */
.component {
    flex-direction: row;
}

@media (max-width: 767px) {
    .component {
        flex-direction: column;
    }
}
```

### 2. Use the Container System

Always wrap page content in containers:

```html
<!-- ✓ Good -->
<section class="section">
    <div class="container">
        <h2>Section Title</h2>
        <p>Content...</p>
    </div>
</section>

<!-- ✗ Bad - No container -->
<section class="section">
    <h2>Section Title</h2>
    <p>Content...</p>
</section>
```

### 3. Test at All Breakpoints

Test your designs at these specific widths:
- **320px** - Minimum phone width (iPhone SE)
- **375px** - Standard phone width (iPhone 12/13)
- **768px** - Tablet portrait (iPad)
- **1024px** - Tablet landscape / Small laptop
- **1440px** - Standard desktop monitor
- **1920px** - Large desktop monitor

### 4. Use Spacing Variables

Always use spacing variables instead of hard-coded values:

```css
/* ✓ Good */
.component {
    padding: var(--spacing-lg);
    margin-bottom: var(--spacing-xl);
}

/* ✗ Bad */
.component {
    padding: 24px;
    margin-bottom: 32px;
}
```

### 5. Touch-Friendly Targets

Ensure interactive elements are large enough for touch:

```css
/* Minimum 44x44px touch targets */
button,
a {
    min-height: 44px;
    min-width: 44px;
    padding: 0.75rem 1.5rem;
}
```

### 6. Readable Line Lengths

Keep line lengths readable (45-75 characters):

```html
<!-- Use container-narrow for long-form content -->
<article class="container-narrow">
    <h1>Article Title</h1>
    <p>Long-form content is easier to read...</p>
</article>
```

## Examples

### Responsive Hero Section

```html
<section class="hero">
    <div class="container">
        <h1>Welcome to Our Site</h1>
        <p class="lead">
            An engaging introduction that scales beautifully across all devices.
        </p>
        <a href="#" class="btn btn-primary">Get Started</a>
    </div>
</section>
```

**Behavior:**
- **Phone**: Hero padding 2rem, h1 28px, lead 18px
- **Tablet**: Hero padding 4rem, h1 32px, lead 20px
- **Desktop**: Hero padding 6rem, h1 36px, lead 22.5px
- **Large**: Hero padding 8rem, h1 48px, lead 24.75px

### Responsive Card Grid

```html
<section class="section">
    <div class="container">
        <h2>Our Features</h2>
        <div class="card-grid">
            <div class="card">
                <h3>Feature 1</h3>
                <p>Description of feature 1...</p>
            </div>
            <div class="card">
                <h3>Feature 2</h3>
                <p>Description of feature 2...</p>
            </div>
            <div class="card">
                <h3>Feature 3</h3>
                <p>Description of feature 3...</p>
            </div>
        </div>
    </div>
</section>
```

**Behavior:**
- **Phone**: 1 column layout, cards stack vertically
- **Tablet**: 2-3 columns depending on content
- **Desktop**: 3 columns
- **Large**: 3 columns with more spacing

### Responsive Navigation

```html
<header>
    <div class="container">
        <nav>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/services">Services</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    </div>
</header>
```

**Behavior:**
- **Phone**: Vertical stack, centered links
- **Tablet+**: Horizontal row with appropriate spacing

### Conditional Content Display

```html
<div class="container">
    <!-- Show only on mobile -->
    <div class="show-phone-only">
        <p>Tap the menu icon to navigate</p>
    </div>
    
    <!-- Hide on mobile -->
    <div class="hide-phone">
        <p>Use the navigation bar above to explore our site</p>
    </div>
</div>
```

### Two-Column Layout

```html
<section class="section">
    <div class="container">
        <div class="row">
            <!-- Full width on phone, half width on tablet+ -->
            <div class="col col-phone-12 col-tablet-6">
                <h3>Column 1</h3>
                <p>Content for first column...</p>
            </div>
            <div class="col col-phone-12 col-tablet-6">
                <h3>Column 2</h3>
                <p>Content for second column...</p>
            </div>
        </div>
    </div>
</section>
```

## Testing Responsive Designs

### Browser DevTools

Use browser developer tools to test responsive designs:

**Chrome/Edge:**
1. Press `F12` or `Cmd+Option+I` (Mac) / `Ctrl+Shift+I` (Windows)
2. Click the device toolbar icon or press `Cmd+Shift+M` / `Ctrl+Shift+M`
3. Select different device presets or enter custom dimensions

**Firefox:**
1. Press `F12` or `Cmd+Option+I` (Mac) / `Ctrl+Shift+I` (Windows)
2. Click the responsive design mode icon or press `Cmd+Option+M` / `Ctrl+Shift+M`

### Test Checklist

- [ ] Test at 320px (minimum phone width)
- [ ] Test at 375px (standard phone)
- [ ] Test at 768px (tablet portrait)
- [ ] Test at 1024px (tablet landscape/laptop)
- [ ] Test at 1440px (desktop)
- [ ] Test at 1920px (large desktop)
- [ ] Test in both portrait and landscape orientations
- [ ] Verify touch targets are at least 44x44px
- [ ] Check that text is readable without zooming
- [ ] Ensure horizontal scrolling is not required
- [ ] Test navigation at all breakpoints
- [ ] Verify images scale properly

## Browser Support

This responsive system supports:

- Chrome/Edge (latest 2 versions)
- Firefox (latest 2 versions)
- Safari (latest 2 versions)
- Mobile Safari (iOS 12+)
- Chrome Mobile (Android 8+)

CSS features used:
- CSS Custom Properties (CSS Variables)
- CSS Grid
- Flexbox
- Media Queries
- Container Queries (progressive enhancement)

## Additional Resources

- [MDN: Responsive Design](https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Responsive_Design)
- [Web.dev: Responsive Web Design Basics](https://web.dev/responsive-web-design-basics/)
- [CSS-Tricks: A Complete Guide to CSS Media Queries](https://css-tricks.com/a-complete-guide-to-css-media-queries/)

---

**Last Updated**: December 18, 2025

