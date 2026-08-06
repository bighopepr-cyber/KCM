# KCM SDK Security Policy

This document covers security policies specific to KCM SDKs. For general security policies, see the root [SECURITY.md](../SECURITY.md).

## Reporting Vulnerabilities

If you discover a security vulnerability in any KCM SDK, please send an email to security@kcm.dev. **Do not report security vulnerabilities through public GitHub issues.**

### What to include

When reporting a vulnerability, please include:

1. Affected SDK language and version
2. Description of the vulnerability
3. Steps to reproduce
4. Potential impact
5. Suggested fix (if any)

### Response timeline

| Stage | Timeline |
|-------|----------|
| Acknowledgment | 24 hours |
| Initial assessment | 72 hours |
| Fix development | 7-14 days |
| Public disclosure | After fix is released |

## Dependency Auditing

### Per-SDK Dependency Tools

| Language | Audit Tool | Command |
|----------|-----------|---------|
| Rust | cargo-audit | `cargo audit` |
| Python | pip-audit | `pip-audit` |
| JavaScript | npm audit | `npm audit` |
| TypeScript | npm audit | `npm audit` |
| Go | govulncheck | `govulncheck ./...` |
| Java | OWASP Dependency-Check | `mvn dependency-check:check` |
| .NET | dotnet list package --vulnerable | `dotnet list package --vulnerable` |
| C | Manual review | N/A |
| C++ | Manual review | N/A |

### Audit Schedule

| Audit Type | Frequency | Tool |
|------------|-----------|------|
| Automated dependency scan | Every CI run | Language-specific tool |
| Manual dependency review | Monthly | Manual |
| Full security audit | Quarterly | External audit |

### Dependency Policy

1. **Minimal dependencies**: Only add dependencies that are essential
2. **Auditable sources**: Only use packages from official registries
3. **Version pinning**: Pin exact versions in lockfiles
4. **Regular updates**: Update dependencies monthly
5. **License compliance**: Only use permissive licenses (MIT, Apache-2.0, BSD)

## Vulnerability Management

### Known Vulnerability Response

| Severity | Response Time | Action |
|----------|--------------|--------|
| Critical | 24 hours | Emergency patch |
| High | 72 hours | Priority fix |
| Medium | 1 week | Scheduled fix |
| Low | 2 weeks | Next release |

### Security Update Process

1. **Discovery**: Vulnerability reported or discovered
2. **Assessment**: Severity and impact evaluated
3. **Fix Development**: Patch developed and tested
4. **Disclosure**: Coordinated disclosure after fix
5. **Release**: Security patch released
6. **Notification**: Users notified via GitHub Security Advisories

## SDK-Specific Security Considerations

### FFI Safety (C, C++, and other native SDKs)

- All FFI functions must validate input pointers
- Memory management must use RAII patterns
- Buffer overflows must be prevented with bounds checking
- Null pointer dereference must be prevented

### Memory Safety (All SDKs)

- Use language-native memory management
- Avoid raw pointer manipulation where possible
- Use bounds checking for array access
- Validate all external inputs

### Input Validation (All SDKs)

- Validate all function parameters
- Sanitize user-provided strings
- Check for integer overflow
- Validate file paths

### Cryptographic Operations

- Use audited cryptographic libraries only
- Never implement custom cryptography
- Use AEAD encryption (AES-256-GCM) for data at rest
- Use CSPRNG for key generation

## Security Best Practices for Contributors

### Code Review

- All security-related changes require security review
- FFI changes require careful review
- Cryptographic changes require expert review

### Testing

- Write security-focused test cases
- Test edge cases and error conditions
- Test with malformed inputs
- Test memory safety under stress

### Documentation

- Document security assumptions
- Document known limitations
- Document safe usage patterns
- Document unsafe operations (if any)

## Compliance

KCM SDKs follow Microsoft security engineering practices:

- Threat modeling for new features
- Security review for all changes
- Automated security testing in CI
- Supply chain security via dependency auditing
