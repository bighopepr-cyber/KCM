# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability within KCM, please send an email to security@kcm.dev. All security vulnerabilities will be promptly addressed.

**Please do not report security vulnerabilities through public GitHub issues.**

### What to include

When reporting a vulnerability, please include:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if any)

### Response timeline

| Stage | Timeline |
|-------|----------|
| Acknowledgment | 24 hours |
| Initial assessment | 72 hours |
| Fix development | 7-14 days |
| Public disclosure | After fix is released |

## Security Best Practices

### For Contributors

- Never commit secrets, API keys, or credentials
- Use `cargo audit` to check for known vulnerabilities
- Follow the principle of least privilege
- Validate all external inputs
- Use AEAD encryption (AES-256-GCM) for data at rest
- Use CSPRNG for key generation

### For Users

- Keep KCM updated to the latest version
- Use encrypted connections for network communication
- Enable audit logging in production
- Follow the principle of least privilege for database access
- Monitor security advisories

## Known Security Considerations

### FFI Safety

The C FFI layer (`kcm-interface`) requires careful handling:

- All FFI functions have null-pointer guards
- Memory management uses `Box::into_raw` / `Box::from_raw`
- All FFI functions have `# Safety` documentation
- Never pass uninitialized memory to FFI functions

### Encryption

KCM uses AES-256-GCM for data encryption:

- Authenticated encryption with associated data
- 256-bit key size
- 96-bit nonce size
- Key derivation via BLAKE3

### Audit Logging

All write operations are audit-logged:

- Hash-chained audit trail
- Tamper-evident design
- FIFO eviction at 100K events
- All events include timestamp, action, and result

## Security Updates

Security updates are released as patch versions (0.0.x). Subscribe to GitHub releases to be notified of security updates.

## Compliance

KCM follows Microsoft security engineering practices:

- Threat modeling for new features
- Security review for all changes
- Automated security testing in CI
- Supply chain security via `cargo audit` / `cargo deny`
