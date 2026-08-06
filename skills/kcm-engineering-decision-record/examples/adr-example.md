# ADR Example

## ADR-011: Use BLAKE3 for Key Derivation

### Status
Accepted

### Context
KCM needs a key derivation function for AES-256-GCM encryption. Options include Argon2, PBKDF2, scrypt, and BLAKE3.

### Decision
Use BLAKE3 for key derivation.

### Rationale
- BLAKE3 is already used for checksums in kcm-storage
- BLAKE3 is faster than Argon2/PBKDF2/scrypt
- BLAKE3 has strong security properties
- Using one hash function reduces dependency surface

### Alternatives Considered

**Argon2** — Memory-hard, but slower and adds dependency
**PBKDF2** — Well-established, but slower and older
**scrypt** — Memory-hard, but slower and adds dependency

### Consequences

**Positive:**
- Consistent hashing across KCM
- Fast key derivation
- No new dependencies

**Negative:**
- Less memory-hard than Argon2
- Newer algorithm, less battle-tested

**Neutral:**
- Key derivation is one-time cost