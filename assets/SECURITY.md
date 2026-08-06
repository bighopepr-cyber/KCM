# assets/ Security Policy

> For project-wide security policies, refer to the [SECURITY.md](../SECURITY.md) located in the repository root.

## Overview

The `assets/` directory contains static files including SVG logos, icons, and images used for documentation and branding. While these files are low-risk, they require basic security hygiene to prevent SVG-based attacks and metadata leakage.

## Security Scope

| Asset Type | Risk Level | Justification |
|------------|------------|---------------|
| SVG files | Low | Vector format, no executable code expected |
| Image files | Low | Static binary data |
| Metadata | Low | May contain creation tool, author info |

## Threat Model

| Threat | Likelihood | Impact | Mitigation |
|--------|-----------|--------|------------|
| SVG XSS via embedded scripts | Low | Medium | Validate SVG files, no `<script>` tags |
| Metadata leakage | Low | Low | Strip metadata before publication |
| Malicious SVG payload | Very Low | Medium | SVG sanitization before merge |
| File size DoS | Very Low | Low | File size limits enforced |

## Security Risks

| Risk | Description | Mitigation |
|------|-------------|------------|
| SVG embedded scripts | JavaScript in SVG files | Scan for `<script>`, `<iframe>`, `on*` attributes |
| SVG external references | References to external resources | Validate no `xlink:href` to external URLs |
| Metadata exposure | Author names, tool versions | Strip metadata in CI |
| Oversized files | Excessive file sizes | Enforce <50KB limit |

## Access Control

| Role | Read | Write | Approve |
|------|------|-------|---------|
| All Contributors | Yes | No | No |
| Core Maintainer | Yes | Yes | Asset changes |
| Documentation Guardian | Yes | Yes | All asset changes |

## RBAC Integration

Asset management follows KCM RBAC:

| Permission Level | Access |
|-----------------|--------|
| Public (0) | Read-only |
| Read (1) | Read-only |
| Write (2) | Propose changes via PR |
| Admin (3) | Approve asset changes |
| SuperAdmin (4) | Direct asset modification |

## Sensitive Assets

| Asset | Sensitivity | Protection |
|-------|-------------|------------|
| `KCM-LOGO.svg` | Medium | Brand integrity, no unauthorized modifications |
| `README.md` | Low | Public information |

## Secret Management

**No secrets are permitted in any asset file.**

| Rule | Enforcement |
|------|-------------|
| No embedded credentials | SVG scan for suspicious patterns |
| No API keys in metadata | Metadata review |
| No internal URLs | URL validation |

## Secure Development Rules

| Rule | Description |
|------|-------------|
| SVG sanitization | Validate SVG files contain no executable code |
| No embedded scripts | Scan for `<script>`, `<iframe>`, `on*` event handlers |
| Validate file sizes | Enforce <50KB limit per file |
| Strip metadata | Remove tool-specific metadata before merge |
| Review SVG content | Manual review of SVG path data |

## Audit Logging

- All asset changes tracked via git history
- PR reviews require maintainer approval
- CI validates SVG safety on every push

## Validation Checklist

Before merging any asset change:

- [ ] No `<script>`, `<iframe>`, or `on*` event handlers in SVGs
- [ ] No external URL references in SVGs
- [ ] File size <50KB per file
- [ ] Metadata stripped (no author, tool version)
- [ ] SVG renders correctly in browsers
- [ ] No sensitive information in file metadata
- [ ] Color palette matches KCM branding

## References

- [SECURITY.md](../SECURITY.md) — Repository root security policy
- [SVG Security Best Practices](https://www.w3.org/TR/SVG/security/)
- `AGENTS.md` — Engineering constitution
- `assets/README.md` — Asset overview
- `assets/KCM-LOGO.svg` — Primary logo
