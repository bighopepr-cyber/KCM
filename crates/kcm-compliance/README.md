# kcm-compliance

Compliance and regulatory standards for KCM: GDPR consent management and data classification.

## Purpose

Implements regulatory compliance features including GDPR data subject rights, consent tracking, and data classification for handling sensitive information.

## Modules

| Module | Purpose |
|--------|---------|
| `gdpr` | GDPR consent management and data subject rights |
| `data_classification` | 4-tier data classification system |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `parking_lot` | Thread-safe state |

## GDPR Features

| Feature | Description |
|---------|-------------|
| Consent Tracking | Record and query consent per data subject |
| Right to Access | Export all facts for a data subject |
| Right to Erasure | Delete all facts for a data subject |
| Consent Withdrawal | Remove consent and trigger erasure |

```rust
use kcm_compliance::gdpr::GDPRManager;

let manager = GDPRManager::new();

// Record consent
manager.record_consent(data_subject_id, consent_type)?;

// Check consent
assert!(manager.has_consent(data_subject_id, consent_type)?);

// Export data subject's facts
let facts = manager.export_subject_data(data_subject_id)?;

// Erase data subject's facts
manager.erase_subject_data(data_subject_id)?;
```

## Data Classification

| Tier | Label | Description | Example |
|------|-------|-------------|---------|
| 0 | Public | No restrictions | Published facts |
| 1 | Internal | Organization-only | Internal knowledge |
| 2 | Confidential | Restricted access | Business rules |
| 3 | Restricted | Encrypted + audited | PII, credentials |

Classification affects:
- Encryption requirements (Tier 3: always encrypted)
- Audit logging (Tier 2+: all access logged)
- Retention policies (configurable per tier)
- Export restrictions (GDPR applies to Tier 3)
