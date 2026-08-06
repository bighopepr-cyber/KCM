# Assets Technical Specification

## Overview

This specification defines the static assets used for KCM branding and documentation, including SVG logos, icons, and images.

## Scope

This specification covers the `assets/` directory contents only. It does not cover documentation files or code assets.

## Responsibilities

| Responsibility | Description |
|----------------|-------------|
| Logo | Primary KCM logo for documentation and branding |
| Branding | Consistent visual identity across all assets |
| Visual identity | Color palette, typography, and design language |

## Technical Specification

### SVG Format

All vector assets use SVG format with the following requirements:

| Property | Requirement |
|----------|-------------|
| Encoding | UTF-8 |
| XML Declaration | Required: `<?xml version="1.0" encoding="UTF-8"?>` |
| SVG Version | 1.1 or later |
| Namespace | `xmlns="http://www.w3.org/2000/svg"` |
| File Size | <50KB per file |

### Color Palette

```yaml
primary_blue: "#0D5EE8"
accent_orange: "#F7AC14"
dark_gray: "#3F4959"
medium_gray: "#6A7685"
light_gray: "#F9F9F9"
background: "#FFFFFF"
```

### File Size Constraints

| File Type | Maximum | Optimized | Rationale |
|-----------|---------|-----------|-----------|
| Logo SVG | 50KB | <30KB | Balance detail and performance |
| Icon SVG | 10KB | <5KB | Simple shapes only |
| Banner SVG | 50KB | <40KB | Wide format optimization |

## Architecture

```
assets/
├── KCM-LOGO.svg      # Primary logo (2012x781)
├── README.md         # Asset documentation
├── SECURITY.md       # Security policy
├── CONTRIBUTING.md   # Contribution guidelines
└── CODE_OF_CONDUCT.md # Community guidelines
```

## Internal Components

### KCM-LOGO.svg

Primary KCM logo with the following properties:

| Property | Value |
|----------|-------|
| Width | 2012px |
| Height | 781px |
| Format | SVG 1.1 |
| Path Count | 46 paths |
| Colors | Blue, Orange, Gray, White |

### Logo Structure

```
KCM-LOGO.svg
├── Background path (#F9F9F9)
├── Letter paths (K, C, M)
├── Accent paths (#F7AC14)
├── Detail paths (#0D5EE8)
└── Shadow paths (#3F4959, #6A7685)
```

## Data Model

### Asset Metadata

```rust
pub struct Asset {
    pub name: String,           // "KCM-LOGO.svg"
    pub format: AssetFormat,    // SVG, PNG
    pub width: u32,             // 2012
    pub height: u32,            // 781
    pub file_size: u64,         // bytes
    pub version: String,        // "1.0"
    pub created: String,        // ISO 8601
    pub modified: String,       // ISO 8601
}

pub enum AssetFormat {
    Svg,
    Png,
}
```

### Color Definition

```rust
pub struct BrandColor {
    pub name: String,           // "primary_blue"
    pub hex: String,            // "#0D5EE8"
    pub rgb: (u8, u8, u8),     // (13, 94, 232)
    pub usage: String,          // "Primary brand color"
}
```

## Execution Flow

### Asset Rendering Flow

```
1. Load SVG file
2. Parse XML structure
3. Resolve color references
4. Render paths to viewport
5. Apply transforms
6. Output to target format
```

### Asset Validation Flow

```
1. Check file exists
2. Validate XML structure
3. Check file size <50KB
4. Scan for executable code
5. Verify color palette
6. Render test at multiple scales
```

## Public API

Assets do not expose a public API. They are consumed by:

- Documentation (Markdown images)
- Website (HTML img tags)
- GitHub (README badges)
- SDKs (branding assets)

## Configuration

### Brand Configuration

```yaml
brand:
  colors:
    primary: "#0D5EE8"
    accent: "#F7AC14"
    dark: "#3F4959"
    medium: "#6A7685"
    light: "#F9F9F9"
  logo:
    min_width: 100
    max_width: 2012
    aspect_ratio: 2.577
  files:
    max_size_kb: 50
    allowed_formats:
      - svg
      - png
```

## Dependencies

| Dependency | Type | Justification |
|-----------|------|---------------|
| SVG renderer | Runtime | Display SVG in browsers |
| XML parser | Runtime | Validate SVG structure |

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Invalid SVG | Malformed XML | Fix SVG structure |
| File too large | Exceeds 50KB | Optimize SVG paths |
| Missing colors | Colors not in palette | Update to brand colors |
| Script detected | Executable code in SVG | Remove script, resubmit |

## Performance Characteristics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Logo load time | <100ms | Browser dev tools |
| Logo file size | <50KB | File system |
| Render time | <16ms | Browser performance |
| Memory usage | <1MB | Browser dev tools |

## Security Considerations

### SVG Safety

| Check | Description | Enforcement |
|-------|-------------|-------------|
| No `<script>` | Prevent XSS | CI scan |
| No `<iframe>` | Prevent embedding attacks | CI scan |
| No `on*` handlers | Prevent event-based attacks | CI scan |
| No external URLs | Prevent data exfiltration | CI scan |
| No `<foreignObject>` | Prevent HTML injection | CI scan |
| File size limit | Prevent DoS | CI check |

### Metadata Stripping

| Metadata | Stripped | Rationale |
|----------|----------|-----------|
| Author | Yes | Prevent information leakage |
| Tool version | Yes | Prevent version fingerprinting |
| Creation date | Yes | Prevent timing analysis |
| Comment | Yes | Reduce file size |

## Integration

Assets integrate with:

```
assets/KCM-LOGO.svg ← docs/README.md
assets/KCM-LOGO.svg ← GitHub repository
assets/KCM-LOGO.svg ← Website
assets/KCM-LOGO.svg ← SDK documentation
```

## Sequence Diagram

### Asset Rendering

```
Browser → Load SVG
  → Parse XML
  → Resolve colors
  → Render paths
  → Display in viewport
```

### Asset Validation

```
CI Pipeline → Load SVG
  → Validate XML
  → Check file size
  → Scan for scripts
  → Verify colors
  → Pass/Fail
```

## Architecture Diagram

```
┌─────────────────────────────────────┐
│           assets/                   │
├─────────┬──────────┬────────────────┤
│ KCM-    │ README   │ Security/      │
│ LOGO.svg│ .md      │ Contributing   │
├─────────┴──────────┴────────────────┤
│     SVG Renderer (Browser)          │
├─────────────────────────────────────┤
│     Documentation / Website         │
└─────────────────────────────────────┘
```

## References

- [SVG Specification](https://www.w3.org/TR/SVG/)
- [SVG Security](https://www.w3.org/TR/SVG/security/)
- [Brand Color Theory](https://www.canva.com/colors/color-meanings/)
- `AGENTS.md` — Engineering constitution
- `assets/README.md` — Asset overview
- `assets/KCM-LOGO.svg` — Primary logo

## SSOT Alignment

| SSOT Requirement | Specification | Implementation | Test |
|-----------------|---------------|----------------|------|
| R-ASSET-001 | SVG format for vector graphics | `assets/*.svg` | Visual review |
| R-ASSET-002 | Consistent color palette | Brand colors defined | Color verification |
| R-ASSET-003 | File size <50KB | `assets/README.md` | CI file size check |
| R-ASSET-004 | No executable code in SVGs | Security policy | CI SVG scan |
| R-ASSET-005 | Metadata stripped | Security policy | CI metadata check |
