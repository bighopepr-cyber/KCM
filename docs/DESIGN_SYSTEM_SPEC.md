# KCM Design System Specification

## Purpose

Defines the complete design language for the KCM Official Website. Every visual element across all pages must conform to this specification.

## Color System

### Light Mode
| Token | Value | Usage |
|-------|-------|-------|
| `--c-bg-0` | `#ffffff` | Page background |
| `--c-bg-1` | `#f6f7f9` | Card/section backgrounds |
| `--c-bg-2` | `#ebedf0` | Hover states, code backgrounds |
| `--c-fg-0` | `#161616` | Primary text, headings |
| `--c-fg-1` | `#525252` | Body text, descriptions |
| `--c-fg-2` | `#8c8c8c` | Muted text, labels |
| `--c-border` | `#e0e0e0` | Primary borders |
| `--c-accent` | `#0066ff` | Interactive elements, links |
| `--c-success` | `#0a7e07` | Status indicators |

### Dark Mode
| Token | Value |
|-------|-------|
| `--c-bg-0` | `#0d0d0d` |
| `--c-bg-1` | `#171717` |
| `--c-bg-2` | `#1f1f1f` |
| `--c-fg-0` | `#e5e5e5` |
| `--c-accent` | `#4d9fff` |

## Typography

| Element | Size | Weight | Letter-spacing |
|---------|------|--------|---------------|
| h1 | 40px | 700 | -0.025em |
| h2 | 28px | 700 | -0.02em |
| h3 | 18px | 600 | normal |
| h4 | 15px | 600 | normal |
| Body | 15px | 400 | normal |
| Small | 14px | 400 | normal |
| Caption | 12px | 400 | 0.06em uppercase |

## Spacing (8px grid)

| Token | Value |
|-------|-------|
| `--sp-1` | 4px |
| `--sp-2` | 8px |
| `--sp-3` | 12px |
| `--sp-4` | 16px |
| `--sp-6` | 24px |
| `--sp-8` | 32px |
| `--sp-12` | 48px |
| `--sp-16` | 64px |

## Border Radius

| Level | Value | Usage |
|-------|-------|-------|
| xs | 3px | Badges, inline elements |
| sm | 4px | Buttons, inputs |
| md | 6px | Code blocks, cards |
| lg | 8px | Cards, panels |

## Elevation

| Level | Shadow | Usage |
|-------|--------|-------|
| 1 | `0 1px 2px rgba(0,0,0,0.06)` | Subtle separation |
| 2 | `0 2px 8px rgba(0,0,0,0.06)` | Dropdown menus |
| 3 | `0 4px 16px rgba(0,0,0,0.08)` | Modals, floating elements |

## Iconography

All icons use inline SVG with:
- 20x20 viewBox
- 1.5px stroke width
- `stroke: currentColor` (no fill)
- `stroke-linecap: round`
- `stroke-linejoin: round`
- Classes: `.icon` (20px), `.icon-sm` (16px), `.icon-lg` (24px)

**No emojis are permitted.** All visual indicators use SVG.

## Components

### Card
Background: `var(--c-bg-1)`, Border: `1px solid var(--c-border)`, Radius: 8px, Padding: 24px.

### Button
| Variant | Background | Border | Text |
|---------|-----------|--------|------|
| btn-primary | `--c-accent` | `--c-accent` | white |
| btn-secondary | transparent | `--c-border` | `--c-accent` |
| btn-ghost | transparent | none | `--c-fg-1` |

### Table
Full-width, collapsed borders. Header: 2px bottom border, 12px uppercase text. Data: 1px bottom border, 14px text.

### Code Block
Background: `--c-code-bg`, Border: `1px solid --c-border`, Radius: 6px, Font: monospace 13px, Line-height: 1.7.

### Status Dot
8x8px circle. Green = passing. No emoji, no text for status.

## Layout

| Breakpoint | Grid | Sidebar |
|-----------|------|---------|
| Desktop (>768px) | Multi-column grid | 224px sticky sidebar |
| Tablet (768px) | Single column | Collapsed |
| Mobile (<480px) | Single column | Hidden |

## Accessibility

- All interactive elements have ARIA labels
- Semantic HTML5 elements used throughout
- Keyboard navigation for all interactive elements
- Color contrast meets WCAG AA (4.5:1 for text)
- `role` attributes on landmark elements
- Skip navigation via keyboard

## Animation Principles

- Duration: 120ms for interactions, 400ms for scroll animations
- Easing: `ease` for interactions, `ease` for scroll
- Only opacity and transform used (GPU-accelerated)
- No animation on text content
- Respects `prefers-reduced-motion`

## Print Styles

Hidden elements: navbar, theme toggle, buttons, mobile menu
Background: white
Text: black
Code blocks: light gray background
