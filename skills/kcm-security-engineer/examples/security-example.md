# Security Validation Example

## Change: Add new FFI function

### Security Check
1. **Null-pointer guard:** Function validates null input ✓
2. **Input validation:** All parameters validated ✓
3. **Memory management:** Uses Box::into_raw/from_raw ✓
4. **Safety docs:** `# Safety` section present ✓
5. **No hardcoded keys:** No secrets in code ✓

### Decision
- **APPROVED** — FFI function meets security requirements
