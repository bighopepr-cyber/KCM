# Contributing to assets/

> For core engine contribution rules, refer to the repository [CONTRIBUTING.md](../CONTRIBUTING.md).

## Overview

This guide covers how to contribute to the KCM static assets including logos, icons, and images. All assets must maintain visual consistency and meet size requirements.

## Before Contributing

1. Verify the asset does not already exist
2. Check the KCM color palette and branding guidelines
3. Ensure the asset serves a clear purpose
4. Open an issue for discussion before adding new asset types

## Coding Standards

### SVG Format

- Use SVG format for all vector graphics
- Use UTF-8 encoding for SVG files
- Include XML declaration: `<?xml version="1.0" encoding="UTF-8"?>`
- Use descriptive `id` attributes for layers

### Color Palette

KCM brand colors:

| Color | Hex | Usage |
|-------|-----|-------|
| Blue | `#0D5EE8` | Primary brand color |
| Orange | `#F7AC14` | Accent color |
| Dark Gray | `#3F4959` | Text and secondary elements |
| Light Gray | `#F9F9F9` | Backgrounds |
| Medium Gray | `#6A7685` | Secondary elements |

### File Size Limits

| File Type | Maximum Size | Rationale |
|-----------|-------------|-----------|
| SVG (logo) | 50KB | Fast loading, reasonable complexity |
| SVG (icon) | 10KB | Simple shapes only |
| PNG | 100KB | Retina-ready at 2x |

## Module Architecture Rules

Assets are organized by type:

| Directory | Content | Purpose |
|-----------|---------|---------|
| `assets/` | Logos, icons, images | Documentation and branding |
| `docs/` | Technical documentation | References assets from `assets/` |

## Documentation Rules

| Rule | Description |
|------|-------------|
| README.md | Document all assets with purpose and format |
| Version awareness | Note which version of branding an asset represents |
| Cross-references | Link to assets from documentation |

## Testing Requirements

| Requirement | Validation |
|-------------|-----------|
| Visual review | Verify rendering in browser and documentation |
| File size check | Enforce <50KB limit |
| SVG safety | No executable code in SVGs |
| Color consistency | Match KCM brand palette |

### Visual Review Process

```
1. Render SVG in browser
2. Verify at multiple scales (1x, 2x, 4x)
3. Check color accuracy against palette
4. Verify no visual artifacts
5. Test in documentation context
```

## Performance Rules

| Rule | Description |
|------|-------------|
| File size <50KB | Keep SVGs simple and optimized |
| Optimize paths | Use SVG optimizer to reduce path complexity |
| Minimize layers | Reduce number of SVG groups/layers |
| Avoid filters | Minimize SVG filter effects |

## Review Checklist

Before submitting an asset PR:

- [ ] Asset serves a clear purpose
- [ ] Format matches existing assets (SVG preferred)
- [ ] File size within limits (<50KB)
- [ ] Colors match KCM brand palette
- [ ] No embedded scripts or external references
- [ ] Metadata stripped
- [ ] Renders correctly at multiple scales
- [ ] README.md updated to include new asset

## Pull Request Requirements

| Requirement | Description |
|-------------|-------------|
| Descriptive title | Clearly state what asset is added/modified |
| Preview | Include rendered preview in PR description |
| Purpose | Explain why the asset is needed |
| Format notes | Specify format and size |
| Reviewer assignment | Tag Documentation Guardian |

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Repository root contribution rules
- `AGENTS.md` — Engineering constitution
- `assets/README.md` — Asset overview
- `assets/KCM-LOGO.svg` — Primary logo reference
