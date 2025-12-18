# Responsive Breakpoints - Visual Guide

## Breakpoint Boundaries

```
0px                768px              1024px             1440px              ∞
│                   │                   │                   │                  │
├───────────────────┼───────────────────┼───────────────────┼──────────────────┤
│                   │                   │                   │                  │
│      PHONE        │      TABLET       │     DESKTOP       │  DESKTOP LARGE   │
│    (Mobile)       │                   │    (Regular)      │                  │
│                   │                   │                   │                  │
│    < 768px        │  768px - 1023px   │ 1024px - 1439px   │     1440px+      │
│                   │                   │                   │                  │
└───────────────────┴───────────────────┴───────────────────┴──────────────────┘
```

## Common Device Examples

```
┌─────────────────────────────────────────────────────────────────────────┐
│  PHONE (< 768px)                                                        │
├─────────────────────────────────────────────────────────────────────────┤
│  • iPhone SE (320px)                                                    │
│  • iPhone 12/13 (375px)                                                 │
│  • iPhone 14 Pro Max (430px)                                            │
│  • Samsung Galaxy (360px - 480px)                                       │
│  • All phones in portrait mode                                          │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  TABLET (768px - 1023px)                                                │
├─────────────────────────────────────────────────────────────────────────┤
│  • iPad (768px portrait, 1024px landscape)                              │
│  • iPad Mini (768px portrait)                                           │
│  • Android tablets (800px - 1000px)                                     │
│  • Small laptops (1024px but feels tablet-like)                         │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  DESKTOP (1024px - 1439px)                                              │
├─────────────────────────────────────────────────────────────────────────┤
│  • MacBook Air/Pro 13" (1280px - 1440px)                                │
│  • Standard laptops (1366px - 1440px)                                   │
│  • Small desktop monitors (1280px)                                      │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│  DESKTOP LARGE (1440px+)                                                │
├─────────────────────────────────────────────────────────────────────────┤
│  • MacBook Pro 16" (1512px - 1728px)                                    │
│  • Full HD monitors (1920px)                                            │
│  • 2K monitors (2560px)                                                 │
│  • 4K monitors (3840px)                                                 │
│  • Ultra-wide displays (2560px - 3440px)                                │
└─────────────────────────────────────────────────────────────────────────┘
```

## Container Max-Widths

```
Viewport Width:    < 768px    768px     1024px    1440px    1920px
                      │          │         │         │         │
                      ▼          ▼         ▼         ▼         ▼
                   ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐
.container         │100%  │  │720px │  │960px │  │1200px│  │1200px│
                   └──────┘  └──────┘  └──────┘  └──────┘  └──────┘

.container-narrow  │100%  │  │640px │  │720px │  │800px │  │800px │
                   └──────┘  └──────┘  └──────┘  └──────┘  └──────┘
```

## Typography Scaling

```
Element         Phone    Tablet   Desktop  Desktop-LG
             ─────────────────────────────────────────
h1              28px     32px     36px      48px
h2              24px     28px     32px      36px
h3              20px     24px     28px      28px
body            16px     16px     18px      18px
.lead           18px     20px     22.5px    24.75px
```

## Spacing Scale Visualization

```
Variable        Phone    Tablet   Desktop  Desktop-LG
             ───────────────────────────────────────────
--spacing-xs    0.25rem  0.25rem  0.25rem   0.25rem   ▪
--spacing-sm    0.5rem   0.5rem   0.5rem    0.5rem    ▪▪
--spacing-md    1rem     1rem     1rem      1rem      ████
--spacing-lg    1.5rem   2rem     2rem      2.5rem    ██████
--spacing-xl    2rem     3rem     3rem      4rem      ████████
--spacing-2xl   3rem     4rem     5rem      6rem      ████████████
--spacing-3xl   4rem     5rem     6rem      8rem      ████████████████
```

## Section Padding

```
Section Type    Phone    Tablet   Desktop  Desktop-LG
             ──────────────────────────────────────────
.section        2rem     4rem     6rem      8rem
.hero           2rem     4rem     6rem      8rem
header          1.5rem   3rem     3rem      4rem
footer          2rem     4rem     5rem      8rem
```

## Grid Behavior

### Card Grid

```
PHONE (< 768px)
┌────────────────────┐
│                    │
│      Card 1        │
│                    │
└────────────────────┘
┌────────────────────┐
│                    │
│      Card 2        │
│                    │
└────────────────────┘
┌────────────────────┐
│                    │
│      Card 3        │
│                    │
└────────────────────┘

TABLET (768px+)
┌─────────┐  ┌─────────┐  ┌─────────┐
│         │  │         │  │         │
│ Card 1  │  │ Card 2  │  │ Card 3  │
│         │  │         │  │         │
└─────────┘  └─────────┘  └─────────┘

DESKTOP (1024px+)
┌───────┐  ┌───────┐  ┌───────┐  ┌───────┐
│       │  │       │  │       │  │       │
│Card 1 │  │Card 2 │  │Card 3 │  │Card 4 │
│       │  │       │  │       │  │       │
└───────┘  └───────┘  └───────┘  └───────┘
```

### Row/Column Grid

```
col-phone-12 col-tablet-6 col-desktop-4

PHONE
┌──────────────────────┐
│      Column 1        │  100% width
└──────────────────────┘
┌──────────────────────┐
│      Column 2        │  100% width
└──────────────────────┘
┌──────────────────────┐
│      Column 3        │  100% width
└──────────────────────┘

TABLET
┌──────────┐┌──────────┐
│Column 1  ││Column 2  │  50% each
└──────────┘└──────────┘
┌──────────────────────┐
│      Column 3        │  100% width
└──────────────────────┘

DESKTOP
┌──────┐┌──────┐┌──────┐
│Col 1 ││Col 2 ││Col 3 │  33.33% each
└──────┘└──────┘└──────┘
```

## Media Query Cascade

```css
/* Base Styles (Phone) */
.element {
    font-size: 16px;
    padding: 1rem;
}

/* Tablet (768px+) */
@media (min-width: 768px) {
    .element {
        font-size: 18px;
        padding: 2rem;
    }
}

/* Desktop (1024px+) */
@media (min-width: 1024px) {
    .element {
        font-size: 20px;
        padding: 3rem;
    }
}

/* Large Desktop (1440px+) */
@media (min-width: 1440px) {
    .element {
        font-size: 24px;
        padding: 4rem;
    }
}
```

## Testing Grid

Use this to test breakpoints in browser:

```
┌────────┬────────┬────────┬────────┬────────┬────────┬────────┐
│  320px │  375px │  768px │ 1024px │ 1440px │ 1920px │ 2560px │
├────────┼────────┼────────┼────────┼────────┼────────┼────────┤
│ iPhone │iPhone  │  iPad  │ Laptop │Desktop │Full HD │  2K    │
│   SE   │12/13   │Portrait│        │        │        │        │
└────────┴────────┴────────┴────────┴────────┴────────┴────────┘
    PHONE      │     TABLET    │   DESKTOP    │  DESKTOP-LG
               │               │              │
         Breakpoint 1    Breakpoint 2   Breakpoint 3
           (768px)         (1024px)       (1440px)
```

## Quick Decision Tree

```
What screen size are you targeting?
│
├─ Phones only?
│  └─ No media query needed (base styles)
│
├─ Tablets and up?
│  └─ @media (min-width: 768px) { ... }
│
├─ Desktop only?
│  └─ @media (min-width: 1024px) { ... }
│
└─ Large screens only?
   └─ @media (min-width: 1440px) { ... }
```

## Container Padding Visualization

```
PHONE (< 768px)
┌─────────────────────────┐
│→1rem                    │  Container padding
│  Content area           │
│                    1rem←│
└─────────────────────────┘

TABLET (768px+)
┌──────────────────────────┐
│→→2rem                    │  Increased padding
│    Content area          │
│                    2rem←←│
└──────────────────────────┘

DESKTOP (1024px+)
┌────────────────────────────┐
│→→→3rem                     │  More breathing room
│     Content area           │
│                     3rem←←←│
└────────────────────────────┘

DESKTOP-LG (1440px+)
┌──────────────────────────────┐
│→→→→4rem                      │  Maximum spacing
│      Content area            │
│                      4rem←←←←│
└──────────────────────────────┘
```

## Utility Classes Quick Map

```
Display Control:
.hide-phone          ✗ ✓ ✓ ✓  (hidden on phone only)
.hide-tablet         ✓ ✗ ✓ ✓  (hidden on tablet only)
.hide-desktop        ✓ ✓ ✗ ✓  (hidden on desktop only)
.hide-desktop-lg     ✓ ✓ ✓ ✗  (hidden on large desktop)

.show-phone-only     ✓ ✗ ✗ ✗  (visible on phone only)
.show-tablet-only    ✗ ✓ ✗ ✗  (visible on tablet only)
.show-desktop-only   ✗ ✗ ✓ ✗  (visible on desktop only)

Legend: ✓ = visible, ✗ = hidden
        Phone Tablet Desktop Desktop-LG
```

## Real-World Usage Pattern

```html
<!-- Navigation adapts to screen size -->
<nav>
    <!-- Hamburger icon on phone -->
    <button class="show-phone-only">☰</button>

    <!-- Full menu on larger screens -->
    <ul class="hide-phone">
        <li><a href="/">Home</a></li>
        <li><a href="/about">About</a></li>
        <li><a href="/contact">Contact</a></li>
    </ul>
</nav>

<!-- Content stacks on phone, side-by-side on tablet+ -->
<div class="row">
    <div class="col col-phone-12 col-tablet-6">Main content</div>
    <div class="col col-phone-12 col-tablet-6">Sidebar</div>
</div>
```

---

**Quick Reference**: See `RESPONSIVE-QUICK-REFERENCE.md` for code snippets
**Full Guide**: See `RESPONSIVE.md` for complete documentation
