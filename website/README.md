# KCM Documentation Website

Static documentation website for the KCM Knowledge Columnar Model.

## Structure

| File/Directory | Purpose |
|---------------|---------|
| index.html | Landing page |
| developer.html | Developer portal |
| dashboard.html | Engineering metrics dashboard |
| css/style.css | Design system (light/dark themes) |
| js/main.js | Theme toggle, mobile menu, smooth scroll |
| images/favicon.svg | SVG favicon |
| docs/ | 16 HTML specification pages |
| robots.txt | Search engine configuration |
| sitemap.xml | Site map for SEO |

## Features

- Dark/light theme support
- Responsive design
- SEO optimized (meta tags, sitemap, robots.txt)
- All 16 technical specifications as HTML pages
- Engineering metrics dashboard

## Deployment

Automatically deployed to GitHub Pages via `.github/workflows/deploy-website.yml`.

Trigger: Push to main branch (website/** paths only).

## Local Development

```bash
# Serve locally
python3 -m http.server 8000 --directory website/

# Open in browser
open http://localhost:8000
```

## Specification Pages

| Page | Source |
|------|--------|
| KCM_API_SPEC.html | docs/KCM_API_SPEC.md |
| KCM_ARCHITECTURE.html | docs/KCM_ARCHITECTURE.md |
| KCM_COLUMNAR_FORMAT_SPEC.html | docs/KCM_COLUMNAR_FORMAT_SPEC.md |
| KCM_COMPRESSION_SPEC.html | docs/KCM_COMPRESSION_SPEC.md |
| KCM_DATA_MODEL_SPEC.html | docs/KCM_DATA_MODEL_SPEC.md |
| KCM_DEPLOYMENT_SPEC.html | docs/KCM_DEPLOYMENT_SPEC.md |
| KCM_ENGINEERING_RULES.html | docs/KCM_ENGINEERING_RULES.md |
| KCM_GLOSSARY.html | docs/KCM_GLOSSARY.md |
| KCM_INDEXING_SPEC.html | docs/KCM_INDEXING_SPEC.md |
| KCM_PERFORMANCE_SPEC.html | docs/KCM_PERFORMANCE_SPEC.md |
| KCM_QUERY_EXECUTION_SPEC.html | docs/KCM_QUERY_EXECUTION_SPEC.md |
| KCM_RUNTIME_SPEC.html | docs/KCM_RUNTIME_SPEC.md |
| KCM_SECURITY_TRUST_SPEC.html | docs/KCM_SECURITY_TRUST_SPEC.md |
| KCM_SPECIFICATION.html | docs/KCM_SPECIFICATION.md |
| KCM_TESTING_SPEC.html | docs/KCM_TESTING_SPEC.md |
| KCM_VERSIONING_SPEC.html | docs/KCM_VERSIONING_SPEC.md |
