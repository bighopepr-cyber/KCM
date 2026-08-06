# doc-link-checker

Markdown link validation tool for the KCM repository.

## Purpose

Validates all internal markdown links across documentation files to ensure no broken references exist.

## Usage

```bash
bash tools/doc-link-checker/check-links.sh
```

## What It Checks

- Internal file links (relative paths)
- Anchor links within documents
- Cross-document references

## What It Skips

- External HTTP/HTTPS links (handled by CI)
- Pure anchor links (#section)
- Mailto links

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All links valid |
| 1 | Broken links found |
