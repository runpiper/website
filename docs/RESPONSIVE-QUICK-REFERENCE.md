# Responsive Design Quick Reference

## Breakpoints

```css
Phone:          < 768px       (Base styles)
Tablet:         768px-1023px  (@media (min-width: 768px))
Desktop:        1024px-1439px (@media (min-width: 1024px))
Desktop Large:  1440px+       (@media (min-width: 1440px))
```

## Container Classes

```html
<div class="container">
    <!-- Main container -->
    <div class="container-narrow"><!-- Narrow container for reading --></div>
</div>
```

**Max widths**: Phone: 100% → Tablet: 720px → Desktop: 960px → Large: 1200px

## Typography Scaling

| Element | Phone | Tablet | Desktop | Large   |
| ------- | ----- | ------ | ------- | ------- |
| h1      | 28px  | 32px   | 36px    | 48px    |
| h2      | 24px  | 28px   | 32px    | 36px    |
| h3      | 20px  | 24px   | 28px    | 28px    |
| body    | 16px  | 16px   | 18px    | 18px    |
| .lead   | 18px  | 20px   | 22.5px  | 24.75px |

## Spacing Variables

```css
--spacing-xs    0.25rem (constant)
--spacing-sm    0.5rem  (constant)
--spacing-md    1rem    (constant)
--spacing-lg    1.5rem → 2rem → 2rem → 2.5rem
--spacing-xl    2rem → 3rem → 3rem → 4rem
--spacing-2xl   3rem → 4rem → 5rem → 6rem
--spacing-3xl   4rem → 5rem → 6rem → 8rem
```

## Grid System

### Card Grid

```html
<div class="card-grid">
    <div class="card">...</div>
</div>
```

**Columns**: Phone: 1 → Tablet: auto-fit → Desktop: auto-fit → Large: auto-fit

### Row/Column Grid

```html
<div class="row">
    <div class="col col-phone-12 col-tablet-6 col-desktop-4">...</div>
</div>
```

**Column classes:**

-   Phone: `col-phone-{12|6}`
-   Tablet: `col-tablet-{12|6|4|3}`
-   Desktop: `col-desktop-{12|6|4|3}`

## Display Utilities

```html
<!-- Hide at breakpoint -->
<div class="hide-phone">Hidden on phone</div>
<div class="hide-tablet">Hidden on tablet</div>
<div class="hide-desktop">Hidden on desktop</div>
<div class="hide-desktop-lg">Hidden on large desktop</div>

<!-- Show only at breakpoint -->
<div class="show-phone-only">Only on phone</div>
<div class="show-tablet-only">Only on tablet</div>
<div class="show-desktop-only">Only on desktop</div>
```

## Text Alignment

```html
<p class="text-center-phone">Centered on phone</p>
<p class="text-center-tablet">Centered on tablet</p>
<p class="text-center-desktop">Centered on desktop</p>
```

## Responsive Spacing

```html
<div class="p-responsive">Responsive padding</div>
<div class="px-responsive">Responsive horizontal padding</div>
<div class="py-responsive">Responsive vertical padding</div>
<div class="m-responsive">Responsive margin</div>
<div class="mx-responsive">Responsive horizontal margin</div>
<div class="my-responsive">Responsive vertical margin</div>
```

## Common Patterns

### Hero Section

```html
<section class="hero">
    <div class="container">
        <h1>Title</h1>
        <p class="lead">Intro text</p>
    </div>
</section>
```

### Content Section

```html
<section class="section">
    <div class="container">
        <h2>Section Title</h2>
        <p>Content...</p>
    </div>
</section>
```

### Card Grid

```html
<div class="card-grid">
    <div class="card">
        <h3>Card Title</h3>
        <p>Card content</p>
    </div>
</div>
```

### Two-Column Layout

```html
<div class="row">
    <div class="col col-phone-12 col-tablet-6">Left</div>
    <div class="col col-phone-12 col-tablet-6">Right</div>
</div>
```

## Test Widths

-   **320px** - Minimum (iPhone SE)
-   **375px** - Standard phone (iPhone 12/13)
-   **768px** - Tablet portrait (iPad)
-   **1024px** - Tablet landscape
-   **1440px** - Desktop
-   **1920px** - Large desktop

## CSS Variables in Custom Styles

```css
.my-component {
    padding: var(--spacing-xl);
    margin-bottom: var(--spacing-lg);
    max-width: var(--container-desktop);
}
```

## Section Padding (Automatic)

| Section  | Phone  | Tablet | Desktop | Large |
| -------- | ------ | ------ | ------- | ----- |
| .section | 2rem   | 4rem   | 6rem    | 8rem  |
| .hero    | 2rem   | 4rem   | 6rem    | 8rem  |
| header   | 1.5rem | 3rem   | 3rem    | 4rem  |
| footer   | 2rem   | 4rem   | 5rem    | 8rem  |

---

**Full documentation**: See `RESPONSIVE.md`
