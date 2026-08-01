# KCM Design System Specification

**Version:** 2.0  
**Status:** Active  
**Scope:** KCM website UI/UX component library and design tokens  

---

## 1. Overview

The KCM Design System is a professional, enterprise-grade visual language for the KCM Knowledge Columnar Model website. It replaces all emoji-based iconography with a stroke-based SVG icon system, establishes a consistent design token architecture, and provides a unified component library across all pages.

### Design Principles

1. **Engineering Precision** — Every element communicates technical rigor
2. **Information Density** — Maximize signal, minimize visual noise
3. **Dark-First** — Dark theme is the primary experience, light theme is an equal alternative
4. **Minimal Surface** — Fewer colors, fewer weights, fewer variations
5. **Accessible** — WCAG 2.1 AA contrast ratios, keyboard navigation, screen reader support

---

## 2. Color System

### Palette: Graphite + Electric Blue

The color system uses a neutral graphite base with a single blue accent.

| Token | Light Value | Dark Value | Usage |
|-------|-------------|------------|-------|
| `--kcm-bg-0` | `#ffffff` | `#0a0a0a` | Page background |
| `--kcm-bg-1` | `#f8f9fa` | `#141414` | Card/section background |
| `--kcm-bg-2` | `#f1f3f5` | `#1a1a1a` | Elevated surface |
| `--kcm-bg-3` | `#e9ecef` | `#262626` | Borders, dividers |
| `--kcm-fg-0` | `#212529` | `#ededed` | Primary text |
| `--kcm-fg-1` | `#495057` | `#a1a1a1` | Secondary text |
| `--kcm-fg-2` | `#868e96` | `#737373` | Muted text |
| `--kcm-fg-3` | `#adb5bd` | `#525252` | Disabled text |
| `--kcm-accent` | `#228be6` | `#339af0` | Interactive elements |
| `--kcm-accent-hover` | `#1c7ed6` | `#5cadf5` | Hover state |
| `--kcm-success` | `#2b8a3e` | `#2b8a3e` | Success indicators |
| `--kcm-warning` | `#e67700` | `#e67700` | Warning indicators |
| `--kcm-danger` | `#c92a2a` | `#c92a2a` | Error indicators |

### Contrast Ratios (Light Mode)

| Combination | Ratio | WCAG Level |
|-------------|-------|------------|
| `--kcm-fg-0` on `--kcm-bg-0` | 13.8:1 | AAA |
| `--kcm-fg-1` on `--kcm-bg-0` | 7.2:1 | AAA |
| `--kcm-accent` on `--kcm-bg-0` | 4.6:1 | AA |
| `--kcm-fg-2` on `--kcm-bg-0` | 4.0:1 | AA (large text) |

---

## 3. Typography

### Font Stacks

| Role | Stack | Fallback |
|------|-------|----------|
| Body | System stack | `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif` |
| Code | Monospace stack | `"SF Mono", "Cascadia Code", "Fira Code", Menlo, Consolas, monospace` |

### Type Scale

| Token | Size | Line Height | Weight | Usage |
|-------|------|-------------|--------|-------|
| `--text-xs` | 12px / 0.75rem | 1.5 | 400 | Labels, badges |
| `--text-sm` | 13px / 0.8125rem | 1.5 | 400 | Secondary text, table cells |
| `--text-base` | 15px / 0.9375rem | 1.6 | 400 | Body text |
| `--text-lg` | 17px / 1.0625rem | 1.5 | 400 | Lead text |
| `--text-xl` | 20px / 1.25rem | 1.3 | 600 | Card headings |
| `--text-2xl` | 24px / 1.5rem | 1.2 | 700 | Section subtitles |
| `--text-3xl` | 32px / 2rem | 1.15 | 700 | Section headings |
| `--text-4xl` | 40px / 2.5rem | 1.1 | 700 | Page titles |

### Letter Spacing

- Headings: `-0.025em` to `-0.02em` (tighter)
- Labels: `0.06em` (uppercase tracking)
- Body: default

---

## 4. Spacing System

4px base grid with predefined tokens:

| Token | Value | Pixels |
|-------|-------|--------|
| `--sp-1` | 0.25rem | 4px |
| `--sp-2` | 0.5rem | 8px |
| `--sp-3` | 0.75rem | 12px |
| `--sp-4` | 1rem | 16px |
| `--sp-5` | 1.25rem | 20px |
| `--sp-6` | 1.5rem | 24px |
| `--sp-8` | 2rem | 32px |
| `--sp-10` | 2.5rem | 40px |
| `--sp-12` | 3rem | 48px |
| `--sp-16` | 4rem | 64px |
| `--sp-20` | 5rem | 80px |
| `--sp-24` | 6rem | 96px |

---

## 5. Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `--r-xs` | 3px | Inline code, badges |
| `--r-sm` | 4px | Buttons, inputs |
| `--r-md` | 6px | Code blocks, cards |
| `--r-lg` | 8px | Feature cards, panels |
| `--r-xl` | 12px | Large cards |
| `--r-full` | 9999px | Pills, dots |

---

## 6. Elevation (Shadows)

| Level | Token | Value | Usage |
|-------|-------|-------|-------|
| 1 | `--shadow-1` | `0 1px 2px rgba(0,0,0,0.06)` | Subtle lift |
| 2 | `--shadow-2` | `0 2px 8px rgba(0,0,0,0.08)` | Cards |
| 3 | `--shadow-3` | `0 4px 16px rgba(0,0,0,0.10)` | Dropdowns, modals |

Dark mode shadows use higher opacity (`0.4`–`0.5`) for visibility on dark backgrounds.

---

## 7. Icon System

### SVG Icon Specifications

- **Viewport:** 24×24
- **Stroke:** `currentColor`
- **Stroke Width:** 1.5px
- **Fill:** `none`
- **Line Cap:** `round`
- **Line Join:** `round`

### Icon Sizes

| Class | Size | Usage |
|-------|------|-------|
| `.icon-sm` | 16×16 | Inline in buttons, nav |
| `.icon` | 20×20 | Default, card headers |
| `.icon-lg` | 24×24 | Feature icons, hero |

### Icon Inventory

| ID | Description | File Reference |
|----|-------------|----------------|
| `icon-storage` | Column/bars | Storage feature |
| `icon-query` | Magnifier with grid | Query feature |
| `icon-reasoning` | Network nodes | Reasoning feature |
| `icon-lock` | Security padlock | Security feature |
| `icon-compress` | Stacked layers | Compression feature |
| `icon-sync` | Circular arrows | Transactions feature |
| `icon-globe` | Globe with meridians | Interfaces feature |
| `icon-chart` | Performance bars | Optimization feature |
| `icon-code` | Terminal brackets | Code/API sections |
| `icon-check` | Checkmark in circle | Status/health |
| `icon-arrow-right` | Navigation arrow | CTAs |
| `icon-sun` | Sun rays | Light theme toggle |
| `icon-moon` | Crescent moon | Dark theme toggle |
| `icon-menu` | Hamburger lines | Mobile menu |
| `icon-search` | Magnifier | Search |
| `icon-copy` | Clipboard | Copy button |
| `icon-chevron` | Dropdown chevron | Expandable sections |
| `icon-github` | GitHub logo | External links |
| `icon-external` | External link | External links |
| `icon-rocket` | Quick start | Quick start sections |
| `icon-shield` | Security shield | Security/compliance |
| `icon-database` | Storage cylinder | Database sections |
| `icon-brain` | Brain network | Reasoning sections |
| `icon-zap` | Lightning bolt | Performance |

### Implementation

Icons are implemented as inline SVG within HTML elements. Each SVG uses:

```html
<svg class="icon" viewBox="0 0 24 24">
  <!-- path/line/circle elements -->
</svg>
```

The `.icon` class applies:
```css
stroke: currentColor;
fill: none;
stroke-width: 1.5;
stroke-linecap: round;
stroke-linejoin: round;
```

---

## 8. Component Library

### 8.1 Navigation Bar

- **Position:** Sticky, top: 0
- **Height:** 56px
- **Background:** `rgba(255,255,255,0.85)` / `rgba(10,10,10,0.88)` with 12px backdrop blur
- **Border:** 1px bottom
- **Active indicator:** 2px accent-colored underline

### 8.2 Buttons

| Variant | Background | Border | Text | Hover |
|---------|-----------|--------|------|-------|
| `.btn-primary` | Accent | Accent | White | Accent-hover |
| `.btn-secondary` | Transparent | Border | Accent | Accent-subtle bg |
| `.btn-ghost` | Transparent | None | fg-1 | fg-0 + bg-2 |

All buttons: 8px 16px padding, `--r-md` radius, 500 weight, 13px text.

### 8.3 Cards

- **Background:** `--kcm-bg-1`
- **Border:** 1px `--kcm-border`
- **Radius:** `--r-lg` (8px)
- **Padding:** `--sp-5` (20px)
- **Hover:** Border transitions to accent color
- **Header:** Flex row with icon (accent-colored) + heading

### 8.4 Feature Cards

Same as cards, with `.card-header` containing a 24px icon and heading. Grid layout: `repeat(auto-fill, minmax(280px, 1fr))`.

### 8.5 Tables

- **Header:** Uppercase, 12px, 600 weight, bg-1 background
- **Cells:** 13px, 12px/16px padding
- **Borders:** 1px bottom on cells, 2px bottom on header
- **Hover:** Accent-subtle background on rows
- **Wrap:** `.table-wrap` for horizontal overflow

### 8.6 Code Blocks

- **Background:** `--kcm-code-bg`
- **Border:** 1px `--kcm-code-border`
- **Font:** Monospace stack, 13px
- **Copy button:** SVG icon + text, appears on hover, 12px

### 8.7 Badges

| Variant | Background | Text |
|---------|-----------|------|
| `.badge-green` | `rgba(43,138,62,0.1)` | `--kcm-success` |
| `.badge-blue` | Accent-subtle | Accent |
| `.badge-gray` | bg-2 | fg-2 |

11px, 600 weight, uppercase, 3px radius.

---

## 9. Layout System

### Page Structure

```
┌─────────────────────────────────────────┐
│  Navbar (sticky, 56px)                  │
├─────────────────────────────────────────┤
│  Hero / Page Header                     │
├─────────────────────────────────────────┤
│  Section (5rem padding)                 │
│  ┌─────────────────────────────────────┐│
│  │  Container (max-w: 1200px)          ││
│  │  Content                            ││
│  └─────────────────────────────────────┘│
├─────────────────────────────────────────┤
│  Footer (4rem top, 2rem bottom)         │
└─────────────────────────────────────────┘
```

### Grid Systems

| Class | Columns | Min Width | Gap |
|-------|---------|-----------|-----|
| `.grid-2` | 2 | 1fr | 16px |
| `.grid-3` | 3 | 1fr | 16px |
| `.grid-4` | Auto-fill | 260px | 16px |
| `.feature-grid` | Auto-fill | 280px | 16px |
| `.docs-grid` | Auto-fill | 260px | 12px |
| `.crate-grid` | Auto-fill | 260px | 12px |
| `.quickstart-grid` | Auto-fit | 320px | 16px |
| `.dashboard-grid` | Auto-fit | 280px | 16px |

### Documentation Layout

Two-column: `240px sidebar` + `1fr content`. Sidebar is sticky with scrollable overflow.

---

## 10. Responsive Breakpoints

| Breakpoint | Width | Changes |
|------------|-------|---------|
| Desktop | > 768px | Full layout |
| Tablet | ≤ 768px | Single column, sidebar becomes static, nav collapses |
| Mobile | ≤ 480px | All grids single column |

### Mobile Navigation

- Nav links hidden, hamburger button visible
- Opens as full-width dropdown below navbar
- Closes on link click or Escape key

---

## 11. Theme System

### Mechanism

- `data-theme` attribute on `<html>` element
- Values: `"light"` (default) and `"dark"`
- Persisted in `localStorage` under key `kcm-theme`
- Respects `prefers-color-scheme` on first visit

### Toggle

- Sun/moon SVG icons in theme button
- Sun shown in dark mode, moon shown in light mode
- 12px square button with border

---

## 12. Animations

### Scroll Reveal

- Elements with `.fade-in` class
- Initial: `opacity: 0; transform: translateY(12px)`
- Visible: `opacity: 1; transform: translateY(0)`
- Trigger: IntersectionObserver at 10% threshold
- Duration: 400ms ease

### Transitions

- Color changes: 120ms
- Border/shadow: 120ms
- Transform: 200ms

---

## 13. Accessibility

### Requirements

- All interactive elements must be keyboard accessible
- SVG icons must have `aria-hidden="true"` when decorative
- Buttons must have `aria-label` when icon-only
- Mobile menu button must have `aria-expanded`
- Focus visible outlines on interactive elements
- Minimum contrast ratio: 4.5:1 for text, 3:1 for large text

### Screen Reader Support

- Nav landmarks with `role="navigation"` and `aria-label`
- Hero with `role="banner"`
- Footer with `role="contentinfo"`
- Charts with `role="img"` and `aria-label`

---

## 14. File Structure

```
website/
├── css/
│   └── style.css          # Complete design system (single file)
├── js/
│   └── main.js            # Theme toggle, mobile menu, search, copy
├── images/
│   ├── favicon.svg        # KCM logo favicon
│   └── icons.svg          # SVG sprite sheet (reference only)
├── index.html             # Landing page
├── dashboard.html         # Engineering dashboard
├── developer.html         # Developer portal
└── docs/                  # Specification documents
```

---

## 15. Naming Conventions

### CSS Classes

- **Block:** `.navbar`, `.hero`, `.footer`, `.card`
- **Element:** `.card-header`, `.hero-sub`, `.nav-links`
- **Modifier:** `.btn-primary`, `.badge-green`, `.section-alt`
- **Utility:** `.icon`, `.icon-sm`, `.icon-lg`, `.fade-in`

### CSS Custom Properties

- **Prefix:** `--kcm-` for all KCM-specific tokens
- **Namespace:** `--kcm-bg-*`, `--kcm-fg-*`, `--kcm-accent-*`
- **Spacing:** `--sp-{n}` where n is the multiplier of 4px

---

## 16. Design Decisions

### Why Inline SVG Instead of Sprite Sheet

Inline SVG ensures:
- No extra HTTP request
- No CORS issues
- Immediate rendering
- `currentColor` inheritance works reliably
- Better accessibility control per icon

### Why Graphite + Blue

- Graphite neutralizes the interface, letting code and data dominate
- Blue (`#228be6`) provides clear interactive signal without competing with content
- Single accent color reduces cognitive load
- High contrast against both light and dark backgrounds

### Why 4px Grid

- Ensures vertical rhythm consistency
- All spacing values are multiples of 4px
- Easy mental model for developers
- Aligns with most font metrics

### Why No CSS Framework

- Full control over output size (single CSS file ~8KB)
- No unused CSS bloat
- Specific KCM design language, not generic
- No build step required
