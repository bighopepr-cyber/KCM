# Debugging Example

## Bug: WAL replay loses data

### Investigation
1. **Reproduce:** Insert 1000 facts, crash, replay → 999 facts
2. **Symptoms:** One fact missing after replay
3. **Environment:** Linux, x86_64, debug build

### Root Cause
- WAL entry for last fact was written to disk but not fsynced
- Crash occurred between write and fsync
- WAL replay skipped incomplete entry

### Fix
- Add fsync after each WAL entry write
- WAL replay skips entries with invalid checksum

### Verification
- Insert 1000 facts, crash, replay → 1000 facts ✓
- All existing tests pass ✓
- No regressions ✓