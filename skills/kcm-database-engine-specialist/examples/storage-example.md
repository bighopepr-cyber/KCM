# Storage Validation Example

## Change: Modify WAL entry format

### Storage Check
1. **Binary Format:** WAL entry format is versioned ✓
2. **WAL Preservation:** All Fact fields preserved ✓
3. **Size:** WAL_INSERT_SIZE updated to new size ✓
4. **Roundtrip:** New format roundtrips correctly ✓
5. **Recovery:** WAL replay works with new format ✓

### Decision
- **APPROVED** — Format change is safe and versioned
