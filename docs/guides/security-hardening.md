# Security Hardening Guide

**Document ID:** GUIDE-SECURITY-001
**Version:** 1.0.0
**Status:** Active
**Owner:** KCM Engineering Team
**Last Reviewed:** 2026-08-04
**Depends on:** KCM_SPECIFICATION, KCM_SECURITY_TRUST_SPEC

## Purpose

This guide covers security configuration including encryption, access control, audit logging, and network security for production KCM deployments.

## Encryption at Rest

Enable AES-256-GCM encryption:

```toml
[security]
encryption_enabled = true
encryption_key_path = "/etc/kcm/encryption.key"
```

### Key Management

```bash
# Generate encryption key
openssl rand -hex 32 > /etc/kcm/encryption.key

# Set permissions
chmod 600 /etc/kcm/encryption.key
chown kcm:kcm /etc/kcm/encryption.key
```

## Encryption in Transit

Enable TLS:

```toml
[server]
tls_enabled = true
tls_cert_path = "/etc/kcm/tls/cert.pem"
tls_key_path = "/etc/kcm/tls/key.pem"
```

### Generate TLS Certificate

```bash
# Self-signed (development)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Production: Use Let's Encrypt or commercial CA
```

## Access Control (RBAC)

Configure Role-Based Access Control:

```toml
[rbac]
enabled = true
admin_users = ["admin"]
default_role = "reader"
```

### Permission Levels

| Level | Permissions |
|-------|------------|
| Reader | Read-only access |
| Writer | Insert, update |
| Delete | Delete facts |
| Execute | Run queries, inference |
| Admin | Full access |

## Audit Logging

Enable hash-chained audit log:

```toml
[audit]
enabled = true
log_path = "/var/log/kcm/audit.log"
max_events = 100000
```

## Network Security

### Firewall Rules

```bash
# Allow only specific IPs
iptables -A INPUT -p tcp --dport 8080 -s 10.0.0.0/8 -j ACCEPT
iptables -A INPUT -p tcp --dport 8080 -j DROP
```

### Kubernetes NetworkPolicy

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: kcm-network-policy
spec:
  podSelector:
    matchLabels:
      app: kcm-server
  policyTypes:
    - Ingress
    - Egress
  ingress:
    - from:
        - podSelector:
            matchLabels:
              app: kcm-client
      ports:
        - port: 8080
```

## Security Checklist

- [ ] Encryption at rest enabled
- [ ] TLS enabled for all connections
- [ ] RBAC configured
- [ ] Audit logging enabled
- [ ] Network policies applied
- [ ] Secrets stored securely (not in code)
- [ ] Regular security audits scheduled
- [ ] Vulnerability scanning in CI
