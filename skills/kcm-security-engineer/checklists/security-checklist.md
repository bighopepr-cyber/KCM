# Security Checklist

## Encryption
- [ ] Uses AES-256-GCM
- [ ] Uses BLAKE3 KDF
- [ ] 256-bit key size
- [ ] 96-bit nonce size
- [ ] No hardcoded keys

## RBAC
- [ ] 5 permission levels enforced
- [ ] No privilege escalation
- [ ] Permission checks on every operation
- [ ] Role hierarchy correct

## Audit Logging
- [ ] Hash-chained audit trail
- [ ] FIFO eviction at 100K events
- [ ] All write operations logged
- [ ] Tamper-evident design

## FFI Security
- [ ] Null-pointer guards on all FFI functions
- [ ] Input validation on all parameters
- [ ] Memory management correct
- [ ] `# Safety` documentation present

## Compliance
- [ ] GDPR consent management works
- [ ] Data classification (4 tiers) enforced
- [ ] PII protection validated
- [ ] Audit trail complete
