# Security Checklist

> Document ID: KCM-CHK-SEC-001 | Version: 1.0.0

## Pre-Implementation

- [ ] Task classified as Security
- [ ] Threat assessment completed
- [ ] P7 Security Engineer involved
- [ ] P4 Specification Lock involved

## Implementation Phase

- [ ] P7 Security Engineer implemented fix
- [ ] Fix follows security best practices
- [ ] No hardcoded keys/tokens/credentials
- [ ] Input validation on all public interfaces
- [ ] Null-pointer guards on all FFI functions
- [ ] CSPRNG for all random number generation

## Cryptographic Requirements

- [ ] AES-256-GCM for encryption at rest
- [ ] BLAKE3 for key derivation
- [ ] 256-bit keys, 96-bit nonces
- [ ] Constant-time comparisons for secrets
- [ ] No timing side-channels

## Testing Phase

- [ ] P9 Testing Verification completed
- [ ] Security tests added
- [ ] Attack surface tested
- [ ] All existing security tests pass

## Audit Trail

- [ ] Hash-chained audit logging
- [ ] FIFO at 100K events
- [ ] All write operations logged
- [ ] Tamper-evident

## Compliance

- [ ] GDPR consent management validated
- [ ] Data classification validated
- [ ] RBAC enforcement validated
- [ ] TLS for network communication

## Documentation

- [ ] SECURITY.md updated (if policy changed)
- [ ] Security advisory created (if CVE)
- [ ] CHANGELOG updated
