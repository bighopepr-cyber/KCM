# Ecosystem Specifications

**Document ID:** KCM-ECO-README-001
**Version:** 2.0.0
**Status:** Active
**Owner:** Product/Platform Owner

This directory defines the repository's current ecosystem-facing documentation. The authoritative source for implementation remains the code and the primary PRD documents; the files in this directory are reference documents for ecosystem integration surfaces.

## Canonical Documents

| Document | Scope |
|----------|-------|
| DEVELOPER_ECOSYSTEM.md | Developer-facing product and integration surface |
| ENTERPRISE_ECOSYSTEM.md | Enterprise deployment and operational posture |
| PLUGIN_SYSTEM.md | Plugin and extension integration model |
| EXTENSION_SYSTEM.md | Runtime and query extension model |
| DEPLOYMENT_STRATEGY.md | Deployment and environment configuration |
| CLOUD_STRATEGY.md | Cloud deployment posture |
| OBSERVABILITY.md | Metrics, logging, and operational visibility |
| LONG_TERM_VISION.md | Repository-level direction and long-range architectural intent |

## Implementation Boundaries

- SDK, CLI, and integration support are documented through the concrete directories under [sdk](../../../sdk), [tools](../../../tools), and [integrations](../../../integrations).
- This directory does not duplicate the lower-level crate or server specifications. It provides coordinate-level context only.
