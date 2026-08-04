# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |
| < 0.1 | No |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** open a public GitHub issue
2. Report security vulnerabilities via GitHub Issues with the 'security' label.
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Response Timeline

- Acknowledgment: Within 48 hours
- Assessment: Within 1 week
- Fix timeline: Depends on severity

## Security Features

### Encryption

- AES-256-GCM for data at rest
- TLS 1.3 for data in transit
- BLAKE3 for key derivation

### Access Control

- RBAC with 5 permission levels
- Context-level ACLs
- Audit logging (hash-chained)

### Data Protection

- GDPR compliance
- Data classification (4 tiers)
- Secure key zeroization

## Security Auditing

- `cargo audit` in CI pipeline
- `cargo deny` for license compliance
- Dependency vulnerability scanning
