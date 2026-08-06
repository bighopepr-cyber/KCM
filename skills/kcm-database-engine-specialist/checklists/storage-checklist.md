# Storage Checklist

## Binary Format
- [ ] Format is deterministic
- [ ] Format is versioned
- [ ] DB_MAGIC present
- [ ] DB_VERSION correct
- [ ] Header layout matches spec

## WAL
- [ ] WAL entries preserve all Fact fields
- [ ] WAL_INSERT_SIZE correct (32 bytes)
- [ ] WAL_DELETE_SIZE correct (12 bytes)
- [ ] WAL replay is idempotent

## Operators
- [ ] All operators skip tombstoned rows
- [ ] All operators handle empty input
- [ ] All operators preserve data types

## Recovery
- [ ] Recovery is complete
- [ ] Recovery is lossless
- [ ] Recovery handles partial writes
- [ ] Recovery handles corruption

## Codecs
- [ ] Roundtrip tests pass
- [ ] Compression ratio meets target (>5x)
- [ ] No data loss in compression/decompression
