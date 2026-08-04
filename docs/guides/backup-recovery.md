# Backup and Recovery Guide

**Document ID:** GUIDE-BACKUP-001
**Version:** 1.0.0
**Status:** Active
**Owner:** KCM Engineering Team
**Last Reviewed:** 2026-08-04
**Depends on:** KCM_SPECIFICATION, KCM_SECURITY_TRUST_SPEC

## Purpose

This guide covers backup strategies, recovery procedures, and disaster recovery planning for KCM deployments.

## Backup Strategy

| Type | Frequency | Retention | Method |
|------|-----------|-----------|--------|
| Full backup | Daily | 30 days | kcm-backup create |
| Incremental | Hourly | 7 days | WAL archival |
| Snapshot | On-demand | Until deleted | kcm-snapshot create |

## Creating Backups

### Full Backup

```bash
# Create backup
kcm-backup create /data/kcm.db

# List backups
kcm-backup list

# Verify backup
kcm-backup verify /backups/kcm_2026-08-03.kcm
```

### Automated Backups (Cron)

```bash
# /etc/cron.d/kcm-backup
0 2 * * * root /usr/local/bin/kcm-backup create /data/kcm.db >> /var/log/kcm-backup.log 2>&1
```

### Kubernetes CronJob

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: kcm-backup
spec:
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: kcm:latest
            command: ["kcm-backup", "create", "/data/kcm.db"]
            volumeMounts:
            - name: data
              mountPath: /data
            - name: backup
              mountPath: /backups
          volumes:
          - name: data
            persistentVolumeClaim:
              claimName: kcm-data
          - name: backup
            persistentVolumeClaim:
              claimName: kcm-backups
          restartPolicy: OnFailure
```

## Restoring from Backup

```bash
# Restore from backup
kcm-restore from /backups/kcm_2026-08-03.kcm /data/kcm.db

# Verify restored data
kcm-doctor check /data/kcm.db
```

## Disaster Recovery

### Recovery Point Objective (RPO)

- Full backup: 24 hours
- WAL archival: 1 hour
- **Effective RPO: 1 hour**

### Recovery Time Objective (RTO)

- Restore from backup: 5-15 minutes
- WAL replay: 1-5 minutes
- **Effective RTO: 15 minutes**

### DR Procedure

1. Stop KCM server
2. Restore from latest backup
3. Replay WAL (if available)
4. Verify data integrity
5. Start KCM server
6. Monitor for issues

## Backup Storage

### Local

```bash
# Store backups on separate disk
/data/kcm.db          # Primary
/backups/kcm*.kcm     # Backups
```

### Cloud (S3)

```bash
# Sync backups to S3
aws s3 sync /backups/ s3://kcm-backups/ --storage-class STANDARD_IA
```

### Cloud (GCS)

```bash
# Sync backups to GCS
gsutil -m rsync -r /backups/ gs://kcm-backups/
```
