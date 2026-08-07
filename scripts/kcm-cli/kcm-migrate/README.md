# kcm-migrate

Schema migration tool for KCM.

## Status: Implemented

The CLI now supports status inspection, applying pending migrations, rolling back the last migration, creating new versioned migration files, validating schema operations, and viewing migration history.

## Commands

| Command | Description |
|---------|-------------|
| kcm-migrate status | Show migration status for the configured migration directory |
| kcm-migrate up | Apply pending migrations using the current knowledge database runtime |
| kcm-migrate down | Roll back the last applied migration step |
| kcm-migrate create <name> | Create a new numbered migration file using a slugified name |
| kcm-migrate validate [count] | Validate schema operations by inserting a number of test facts |
| kcm-migrate history | Display the migration history from the version file |

## Usage

```bash
# Check status
kcm-migrate status

# Apply migrations
kcm-migrate up

# Create a new migration
kcm-migrate create "Add index"

# Validate schema behavior
kcm-migrate validate 100
```

## Notes

- Migration files are created under the directory passed with `--dir` (default: `kcm_migrations`).
- Created filenames follow the convention `NNN_slugified_name.sql`.
- The tool stores the current migration version in a `version` file inside the migration directory.
