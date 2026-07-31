## GAP DAN BAGIAN SENSITIF UNTUK PENGEMBANGAN KCM

---

### **I. IMPLEMENTATION GAPS (Features Planned tapi Belum Ada)**

#### **A. Query Engine Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Full SQL/KQL Parser** | HIGH | Cannot execute queries programmatically | Parser exists tapi grammar incomplete, no validation |
| **Aggregate DISTINCT** | HIGH | Cannot count unique values | Missing aggregate function |
| **GROUP BY semantics** | CRITICAL | Cannot do aggregation per group | Fundamental for analytics |
| **Window Functions** | MEDIUM | No ranking, running totals | Not mentioned in architecture |
| **CTE (Common Table Expressions)** | MEDIUM | Cannot compose complex queries | Advanced feature, lower priority |
| **UNION / EXCEPT / INTERSECT** | MEDIUM | Cannot combine result sets | Set operations missing |
| **Subqueries in WHERE** | HIGH | Limited query expressiveness | Not stated as supported |
| **Multi-table JOINs (3+)** | HIGH | Only 2-table joins likely | Join operator incomplete |

**Action Required:**
- Define KQL formal grammar (EBNF notation)
- Implement parser with error recovery
- Add optimizer rules untuk query rewriting
- Create integration tests per operator combination

---

#### **B. Optimizer Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Cost Model Definition** | CRITICAL | No optimization happens | "Cost-based" claimed tapi undefined |
| **Cardinality Estimation** | CRITICAL | Cannot estimate output size | Essential for join ordering |
| **Statistics Collection** | HIGH | Cannot estimate predicates | Need column histograms |
| **Join Ordering Algorithm** | HIGH | Fixed join order, suboptimal | Critical for multi-table queries |
| **Index Selection Logic** | HIGH | Manual index choice only | Should be automatic |
| **Query Plan Caching** | MEDIUM | Recompile per query | Performance issue |
| **Hints/Directives Support** | MEDIUM | User cannot override planner | Sometimes necessary |

**Action Required:**
```rust
// Define CostModel trait:
pub trait CostModel {
    fn estimate_cpu(&self, rows: u64, width: usize) -> f64;
    fn estimate_io(&self, rows: u64) -> f64;
    fn estimate_memory(&self, rows: u64) -> usize;
}

// Cardinality estimation:
pub trait CardinalityEstimator {
    fn estimate_scan(&self, column: ColumnID) -> u64;
    fn estimate_filter(&self, col: ColumnID, predicate: &Predicate) -> f64; // selectivity
    fn estimate_join(&self, left: u64, right: u64, join_key: ColumnID) -> u64;
}
```

---

#### **C. Persistence & Recovery Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Full ACID Guarantee Proof** | CRITICAL | Cannot guarantee consistency | Documented but not proven |
| **Crash Recovery Test** | HIGH | Untested failure mode | Must test power loss scenarios |
| **Incremental Backup Validation** | HIGH | Corrupt backups possible | Restore testing incomplete |
| **Point-in-Time Recovery** | MEDIUM | Cannot recover to snapshot | Useful for compliance |
| **Concurrent Transaction Semantics** | HIGH | MVCC vs locking unclear | Isolation level undefined |
| **Write Conflict Resolution** | MEDIUM | OCC vs pessimistic? | Strategy not stated |
| **Durability Guarantee (fsync policy)** | CRITICAL | Unclear data safety | WAL strategy not explicit |

**Critical Question:** Apakah WAL fsync-per-write atau batch fsync?

---

#### **D. Distributed Mode Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Sharding Strategy Details** | CRITICAL | Incomplete spec | Hash/Range/ConsistentHash stated tapi impl unclear |
| **Data Replication** | CRITICAL | No HA (High Availability) | Single node only currently |
| **2PC Coordinator Impl** | HIGH | Distributed txns unproven | Specification exists tapi code minimal |
| **Cross-shard Joins** | HIGH | Cannot join across shards | Major limitation |
| **Shard Rebalancing** | MEDIUM | Growth/shrinking untested | Operational nightmare |
| **Network Partition Handling** | CRITICAL | CAP theorem unresolved | CP or AP choice not clear |
| **Consensus Protocol** | CRITICAL | No multi-leader setup | Cannot share leadership |

**Action Required:**
- Define quorum write/read explicitly
- Implement gossip protocol untuk membership
- Add partition detection + recovery
- Test network splits (Jepsen style)

---

#### **E. Reasoning Engine Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Rule Discovery Mechanism** | MEDIUM | "Association mining" vague | Algorithm not specified |
| **Negation Handling** | HIGH | Can cause semantic issues | Closed-world assumption? |
| **Recursive Rules** | MEDIUM | Cannot express relationships | Fixed-point iteration needed |
| **Stratification Semantics** | MEDIUM | Unclear iteration order | May cause non-determinism |
| **Constraint Checking** | MEDIUM | No validation di inference | Can infer invalid facts |
| **Explanation Generation** | HIGH | Cannot explain why inferred | Required for audit trail |
| **Performance Bounds** | HIGH | Infinite loop possible | Max iterations only safeguard |

**Action Required:**
```rust
// Add explanation/provenance tracking:
pub struct Derivation {
    pub derived_fact: Fact,
    pub rule_applied: RuleID,
    pub source_facts: Vec<RowID>,  // Why this was inferred
    pub confidence_chain: Vec<f64>, // How confidence computed
}
```

---

#### **F. ML/Learned Index Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Model Training Logic** | CRITICAL | No impl for learned index | Feature incomplete |
| **Online Learning** | MEDIUM | Static models only | Cannot adapt to workload |
| **Model Eviction** | MEDIUM | Memory unbounded | Can OOM |
| **Confidence Learner** | MEDIUM | Not specified clearly | How to train on facts? |
| **Feature Engineering** | MEDIUM | What features used? | Unclear |

---

#### **G. Compliance & GDPR Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **Data Subject Deletion Atomicity** | CRITICAL | Can leave orphaned refs | Consistency risk |
| **Consent Management** | HIGH | Enforcement unclear | Just tracking or enforcement? |
| **Retention Policy Execution** | HIGH | Data not actually deleted | Compliance failure |
| **Data Portability Format** | MEDIUM | Export format undefined | GDPR requirement |
| **Privacy Impact Assessment** | MEDIUM | No PIA template | Doc requirement |
| **Audit Trail Immutability** | HIGH | Audit logs mutable? | Compliance requirement |
| **Data Classification Enforcement** | HIGH | Tagging only, no action | Classification without enforcement |

**Critical:** GDPR requires provable deletion. Current impl unclear how deletion propagates.

---

#### **H. Monitoring & Observability Gaps**

| Gap | Severity | Impact | Notes |
|-----|----------|--------|-------|
| **No Prometheus Metrics** | MEDIUM | Cannot monitor production | Zero visibility |
| **No Structured Logging** | MEDIUM | Hard to debug issues | Unstructured logs only |
| **No Distributed Tracing** | MEDIUM | Cannot trace across services | Single-node only mitigates |
| **No Health Check Endpoint** | HIGH | Kubernetes cannot detect failure | Load balancer blind |
| **No SLO/SLI Definitions** | MEDIUM | Cannot measure reliability | Business metrics missing |
| **No Error Rate Alerting** | MEDIUM | Silent failures possible | Ops blind |
| **No Performance Profiling Hooks** | MEDIUM | Cannot identify bottlenecks | Perf debugging hard |

---

### **II. ARCHITECTURAL GAPS (Design Issues)**

#### **A. Type System Limitations**

```
ISSUE 1: PredicateID = u8 (max 256 predicates)
├─ Problem: Knowledge graphs often have 1000s of relation types
├─ Example: Wikidata has 8000+ properties
├─ Impact: Schema evolution bottleneck, cannot merge knowledge bases
├─ Fix: Escalate to u32 (4B predicates)
└─ Migration: Requires full database rewrite

ISSUE 2: OwnerID = u16 (max 65K owners)
├─ Problem: Organization with 100K users cannot use per-fact ownership
├─ Impact: RBAC granularity lost for large deployments
├─ Fix: Escalate to u32
└─ Migration: Schema version bump

ISSUE 3: Timestamp = i64 (nanoseconds)
├─ Problem: Overflows in year 2262 (Java's same issue)
├─ Impact: Long-running systems will break
├─ Fix: Use u128 or datetime library
└─ Timeline: Address before year 2200
```

#### **B. Storage Layer Blind Spots**

```
ISSUE 4: No Separation of Metadata & Data
├─ Problem: 10 columns all treated equally
├─ Impact: Cannot efficiently update single attribute
├─ Example: Change confidence of 1 fact = read all 10 columns
├─ Fix: Columnar updates with partial flushes
├─ Complexity: High

ISSUE 5: No Multi-Version Concurrency (MVCC)
├─ Problem: Readers block writers during flush
├─ Impact: Write throughput severely limited
├─ Current Model: Write lock for all modifications
├─ Fix: Implement snapshot isolation
├─ Complexity: Very High

ISSUE 6: No Adaptive Compression
├─ Problem: Codec fixed at column creation
├─ Impact: Cannot optimize for workload changes
├─ Example: Predicate column becomes sparse over time
├─ Fix: Monitor compression ratios, auto-recompress
└─ Complexity: Medium

ISSUE 7: WAL Fsync Strategy Unclear
├─ Problem: Group commit not mentioned
├─ Impact: Write throughput limited if fsync per write
├─ Risk: Data loss if crash before fsync
├─ Action: Document and benchmark fsync strategy
└─ Options: Per-write, batch, async
```

#### **C. Query Execution Gaps**

```
ISSUE 8: No Predicate Pushdown Strategy
├─ Problem: Filter operator placement not optimized
├─ Impact: Can scan 1M rows to return 10
├─ Fix: Explicit pushdown rules in optimizer
└─ Complexity: Medium

ISSUE 9: No Vectorization of Expressions
├─ Problem: SIMD only for simple range predicates
├─ Impact: Complex filters (A AND (B OR C)) not vectorized
├─ Fix: JIT compile filter expressions
└─ Complexity: High

ISSUE 10: No Adaptive Execution
├─ Problem: Plan chosen at compile-time only
├─ Impact: Cardinality mismatch = suboptimal query
├─ Fix: Runtime feedback loop
└─ Complexity: Very High
```

---

### **III. SENSITIVE AREAS (HIGH-RISK CODE PATHS)**

#### **A. CRITICAL: Unsafe Code Blocks**

**Location 1: DenseVec allocation (src/vec.rs)**
```rust
// ⚠️ SENSITIVE: Manual memory management
unsafe impl<T: Copy + Send> Send for DenseVec<T> {}
unsafe impl<T: Copy + Send + Sync> Sync for DenseVec<T> {}

// ⚠️ SENSITIVE: Layout calculation
let layout = Layout::from_size_align(
    capacity * std::mem::size_of::<T>(),
    alignment.max(std::mem::align_of::<T>()),
)?;

// ⚠️ SENSITIVE: Raw pointer allocation
let ptr = unsafe { alloc(layout) } as *mut T;

// ⚠️ SENSITIVE: Index access
unsafe {
    *self.ptr.as_ptr().add(self.len) = value;
}
```

**Risks:**
- Integer overflow dalam `capacity * size_of::<T>()`
  - Example: capacity=1M, T=u64 (8 bytes) = 8M bytes OK
  - Example: capacity=u32::MAX, T=u64 = overflow!
- Alignment mismatch jika T tidak properly aligned
- Use-after-free jika clone() fails partway through

**Mitigation:**
```rust
// Add overflow check:
let bytes = capacity.checked_mul(std::mem::size_of::<T>())
    .ok_or("Capacity overflow")?;

// Add alignment validation:
debug_assert_eq!(std::mem::align_of::<T>() <= alignment, true);
```

---

**Location 2: Column compression (src/column.rs)**
```rust
// ⚠️ CRITICAL: Unsafe slice casting
let byte_slice = unsafe {
    std::slice::from_raw_parts(
        slice.as_ptr() as *const u8,
        std::mem::size_of_val(slice)
    )
};

// ⚠️ CRITICAL: Decompression without validation
unsafe {
    std::ptr::copy_nonoverlapping(
        decompressed.as_ptr(),
        ptr as *mut u8,
        decompressed.len().min(expected),  // ⚠️ SILENT TRUNCATION!
    );
}
```

**Risks:**
- Unaligned read jika T has non-standard alignment
- Data loss jika decompressed size < expected
- Uninitialized memory jika copy partial
- Buffer overflow jika decompressed size > expected + ptr buffer

**Must Fix:**
```rust
// BEFORE: Silent truncation
decompressed.len().min(expected)

// AFTER: Explicit validation
if decompressed.len() != expected {
    return Err(KcmError::Corrupted(
        format!("Decompression size mismatch: got {}, expected {}",
                decompressed.len(), expected)
    ));
}
```

---

**Location 3: Bitmap operations (src/bitmap.rs)**
```rust
// ⚠️ SENSITIVE: Bit manipulation
pub fn set(&mut self, idx: usize) {
    let word_idx = idx / 64;
    let bit_idx = idx % 64;
    self.words[word_idx] |= 1 << bit_idx;  // ⚠️ No bounds check!
}
```

**Risk:** Out-of-bounds access jika idx >= capacity * 64

---

#### **B. CRITICAL: Confidence Calculus**

```rust
// ⚠️ SENSITIVE: Floating-point arithmetic
pub fn multiply(&self, other: Confidence) -> Confidence {
    let product = (self.0 * other.0).clamp(0.0, 1.0);
    Confidence(product)  // ⚠️ Constructor doesn't validate!
}
```

**Risks:**
1. Floating-point underflow dengan very small numbers:
   - 0.1 * 0.1 * 0.1 * ... = 1e-100 (denormalized)
   - Precision loss accumulates
   
2. NaN propagation jika one operand NaN:
   - Valid confidence * NaN = NaN
   - Constraint (0.0..=1.0) violated!
   - But constructor bypassed via Confidence::multiply()

3. Disjunction formula can overflow:
   - P(A∨B) = P(A) + P(B) - P(A)·P(B)
   - 0.9 + 0.9 - 0.9*0.9 = 0.99 (OK)
   - 1.0 + 1.0 - 1.0*1.0 = 1.0 (edge case)
   - But what about 0.999 + 0.999 = 1.998?
   - Clamped to 1.0 (lossy!)

**Mitigation:**
```rust
pub fn multiply(&self, other: Confidence) -> Result<Confidence, String> {
    let product = (self.0 * other.0).clamp(0.0, 1.0);
    // Validate result doesn't violate constraints
    Confidence::new(product)  // ✅ Run constructor validation
}
```

---

#### **C. CRITICAL: Transaction & Concurrency**

```rust
// ⚠️ SENSITIVE: Lock acquisition pattern
pub fn insert(&mut self, fact: Fact) -> Result<RowID, KcmError> {
    // Assuming: self = Arc<RwLock<Schema>>
    let mut schema = self.write();  // ⚠️ Blocking acquire!
    
    // What happens if:
    // 1. Thread A: write-locks for 10 seconds (long operation)
    // 2. Thread B: wants to read (blocked!)
    // 3. Thread C: wants to insert (queued)
    // Result: All readers starved while A runs
}
```

**Risk:** Write lock can starve readers indefinitely

- No timeout
- No queue fairness
- FIFO order not guaranteed

**Mitigation:**
```rust
// Option 1: Async with timeout
tokio::time::timeout(
    Duration::from_secs(10),
    schema.write()
).await?

// Option 2: Reader-friendly write (Copy-on-Write)
// Keep readers on old version during write
```

---

**Lock during flush (storage layer)**
```rust
// ⚠️ SENSITIVE: Holding lock during I/O
pub fn flush(&mut self) -> Result<(), KcmError> {
    let _lock = self.lock.write();  // ⚠️ Held during entire flush!
    
    // Compress columns (slow!)
    for col in &mut self.columns {
        col.compress_data()?;  // Could be seconds
    }
    
    // Write to disk (slow!)
    self.write_to_file()?;  // Could be 100ms+
    
    // Lock held entire time - readers blocked!
}
```

**Risk:** Flush operation can block all reads

- Compression is CPU-intensive
- Disk I/O is slow
- No concurrent reads possible

**Mitigation:**
```rust
// Option 1: Background flush thread (async)
// Option 2: Copy data, release lock, then compress
```

---

#### **D. CRITICAL: Reasoning Engine Inference**

```rust
// ⚠️ SENSITIVE: Iterative rule application
pub fn forward_chain(&mut self, rules: &[Rule]) -> Result<(), KcmError> {
    for iteration in 0..MAX_ITERATIONS {  // ⚠️ Only safeguard is iteration limit
        let mut new_facts = Vec::new();
        
        for rule in rules {
            // Apply rule to all facts
            let matches = self.match_rule(rule)?;
            
            for matched in matches {
                let derived = self.apply_rule(rule, &matched)?;
                // ⚠️ What if derived fact already exists?
                // ⚠️ What if derived fact violates constraints?
                new_facts.push(derived);
            }
        }
        
        if new_facts.is_empty() {
            break;  // Fixed point reached
        }
        
        // ⚠️ No deduplication!
        self.insert_batch(&new_facts)?;
    }
}
```

**Risks:**
1. Duplicate inference: Same fact derived multiple times
   - Wastes space
   - Confidence not aggregated correctly
   
2. Constraint violation: Derived fact invalid
   - Example: confidence > 1.0 (impossible)
   - No validation before insert
   
3. Infinite loops: Non-terminating rules
   - MAX_ITERATIONS only protects memory
   - CPU time unbounded
   - No timeout

4. Negation as failure semantics:
   - Rule: NOT(married(X)) → eligible(X)
   - Closed-world assumption? Open-world?
   - If assumption wrong = incorrect inference

**Mitigation:**
```rust
pub fn forward_chain(&mut self, rules: &[Rule]) -> Result<InferenceStats, KcmError> {
    let start = Instant::now();
    let max_duration = Duration::from_secs(60);  // ✅ Timeout
    
    for iteration in 0..MAX_ITERATIONS {
        if start.elapsed() > max_duration {
            return Err(KcmError::Timeout("Inference exceeded time limit"));
        }
        
        let mut new_facts = Vec::new();
        for rule in rules {
            let matches = self.match_rule(rule)?;
            for matched in matches {
                let derived = self.apply_rule(rule, &matched)?;
                // ✅ Validate derived fact
                Confidence::new(derived.confidence)?;
                // ✅ Check for duplicates
                if !self.fact_exists(&derived) {
                    new_facts.push(derived);
                }
            }
        }
        
        if new_facts.is_empty() { break; }
        self.insert_batch(&new_facts)?;
    }
    
    Ok(InferenceStats { iterations: iteration, ... })
}
```

---

#### **E. CRITICAL: File Format & Compatibility**

```rust
// ⚠️ SENSITIVE: Binary file format versioning
const MAGIC: &[u8] = b"KCM\x00";
const VERSION: u32 = 1;  // ⚠️ Only 1 version?

pub fn write_header(&self) -> Result<Vec<u8>, KcmError> {
    let mut buf = Vec::new();
    buf.extend_from_slice(MAGIC);
    buf.extend_from_slice(&VERSION.to_le_bytes());
    // ⚠️ What if format changes?
    // ⚠️ Can v2 read v1 files?
    // ⚠️ Backward compatibility plan?
}
```

**Risks:**
1. No forward compatibility
   - v2 software cannot read v1 files = migration required
   
2. No field versioning
   - Adding column in v2 breaks v1 readers
   
3. No checksum for structure
   - Corrupt file format not detected until parse fails
   
4. Endianness assumed
   - LE_BYTES OK for single system, bad for transfers

**Mitigation:**
```rust
pub struct FileHeader {
    magic: [u8; 4],           // b"KCM\x00"
    version: u32,             // 1, 2, 3, ...
    schema_hash: u64,         // Detect structure changes
    num_columns: u32,         // Validate column count
    row_count: u64,           // Sanity check
    compression_codec: u8,    // Per-file codec
    endianness: u8,           // 0=LE, 1=BE
    reserved: [u8; 32],       // Future expansion
    checksum: u64,            // Blake3 hash of header
}
```

---

#### **F. HIGH: Dictionary Encoding Collisions**

```rust
// ⚠️ SENSITIVE: String → u32 mapping
pub fn insert(&mut self, value: &str) -> u32 {
    if let Some(&id) = self.str_to_id.get(value) {
        return id;  // ⚠️ What if u32::MAX reached?
    }
    
    let id = self.next_id;
    self.next_id += 1;  // ⚠️ Overflow not checked!
    
    self.id_to_str.insert(id, value.to_string());
    self.str_to_id.insert(value.to_string(), id);
    
    id
}
```

**Risk:** Dictionary saturation
- Max 4B unique values
- After 4B distinct strings = what happens?
- Overflow silently? Error?

**Mitigation:**
```rust
pub fn insert(&mut self, value: &str) -> Result<u32, KcmError> {
    if let Some(&id) = self.str_to_id.get(value) {
        return Ok(id);
    }
    
    let id = self.next_id.checked_add(1)
        .ok_or(KcmError::OutOfMemory)?;  // ✅ Explicit check
    
    self.next_id = id;
    // ...
    Ok(id)
}
```

---

### **IV. FRAGILE ASSUMPTIONS (Things that must remain true)**

```
ASSUMPTION 1: T is always Copy
├─ Implication: No complex types in columns (OK - integers/floats only)
├─ Risk: Someone adds String column = memory unsafety
├─ Safeguard: Generic constraint T: Copy (enforced at compile-time ✅)
└─ Trust: Developers understand columnar model

ASSUMPTION 2: RowID is sequential
├─ Implication: No gaps in row numbering (0, 1, 2, ...)
├─ Risk: Soft deletes (tombstones) break this assumption
├─ Current: Bitmap tracking but not enforced
├─ Action: Add invariant check in tests

ASSUMPTION 3: Dictionary IDs are stable
├─ Implication: Once string → id, always same id
├─ Risk: Serialization/deserialization must preserve ids
├─ Sensitive: Backup/restore, cross-node replication
├─ Action: Version dictionary with hash

ASSUMPTION 4: Fact attributes don't change during inference
├─ Implication: Concurrent reads safe during write
├─ Risk: Reader sees torn write (half-updated)
├─ Current: Write lock prevents this ✅
├─ Risk if changed: Remove lock = data races

ASSUMPTION 5: Confidence always in [0.0, 1.0]
├─ Implication: No sentinel values, no NaN
├─ Risk: Floating-point errors violate assumption
├─ Current: Validated in constructor only
├─ Risk: multiply()/combine_or() bypass validation
├─ Action: Add assertions in operators

ASSUMPTION 6: Encoding/Codec is lossless
├─ Implication: Decompress(Compress(data)) == data exactly
├─ Risk: Gorilla encoding for confidence can lose precision
├─ Current: Not mentioned in docs
├─ Action: Document precision guarantees
```

---

### **V. OPERATIONAL RISKS**

#### **A. Production Deployment Blind Spots**

```
RISK 1: No Health Check Endpoint
├─ Symptom: Kubernetes doesn't know if service alive
├─ Impact: Failed instances not evicted, traffic drops
├─ Fix: Implement /health endpoint
├─ Required Fields: {status: OK/ERROR, db_size: bytes, query_latency_ms: f64}
└─ Latency: Must respond in < 100ms

RISK 2: No Graceful Shutdown
├─ Symptom: Kill -9 in middle of flush = corruption
├─ Impact: Data loss, recovery required
├─ Fix: Implement shutdown signal handler
│   ├─ Stop accepting writes
│   ├─ Wait for in-flight transactions (30s timeout)
│   ├─ Flush all columns
│   ├─ Close files
│   └─ Exit
└─ Time Budget: 60 seconds total (k8s terminationGracePeriodSeconds)

RISK 3: No Connection Pooling
├─ Symptom: Clients exhaust file descriptors
├─ Impact: New connections refused
├─ Fix: Implement connection pool (max 1000 default)
└─ Monitoring: Track active/idle connections

RISK 4: No Memory Limits
├─ Symptom: OOM kill when cache grows unbounded
├─ Impact: Service killed abruptly
├─ Fix: Implement memory budget
│   ├─ Track allocated memory
│   ├─ LRU eviction when limit hit
│   └─ Return error if no evict possible
└─ Default: 1GB cache limit
```

---

#### **B. Backup & Disaster Recovery Risks**

```
RISK 5: No Incremental Backup Validation
├─ Symptom: Backup appears OK, restore fails halfway
├─ Impact: No usable backup when needed
├─ Fix: Implement checksum validation
│   ├─ Blake3 hash per column in backup
│   ├─ Manifest file lists hashes
│   └─ Validate before/after restore
└─ Test: Restore backup to test instance monthly

RISK 6: No Cross-Region Replication
├─ Symptom: Data center fails = total loss
├─ Impact: Complete data loss, no recovery
├─ Fix: Async replication to standby region
│   ├─ Primary sends WAL to replica
│   ├─ Replica applies asynchronously
│   ├─ Promote replica on primary failure
│   └─ RPO: Near-zero, RTO: Minutes
└─ Network: Separate network link for replication

RISK 7: No Automatic Failover
├─ Symptom: Primary down = manual intervention needed
├─ Impact: Downtime hours, data staleness
├─ Fix: Implement leader election
│   ├─ Etcd/Zookeeper for consensus
│   ├─ Health check quorum
│   └─ Auto-promote follower
└─ Time: < 30 seconds failover latency
```

---

### **VI. DEVELOPMENT PROCESS RISKS**

#### **A. Testing Gaps**

```
RISK: No Jepsen-Style Testing
├─ Current: Unit tests, load tests only
├─ Missing: Network partition simulation
├─ Impact: Distributed mode untested under failure
├─ Fix: Use Jepsen or similar tool
├─ Scenarios:
│   ├─ Network partition (split brain)
│   ├─ Node crash during commit
│   ├─ Slow network (high latency)
│   ├─ Byzantine failures (data corruption)
│   └─ Clock skew (NTP failure)
└─ Frequency: Monthly comprehensive test

RISK: No Fuzzing
├─ Current: Property tests exist tapi not continuous
├─ Missing: Fuzz binary format parsing, KQL parser
├─ Impact: Crash-prone parsing of untrusted input
├─ Fix: Integrate cargo-fuzz
│   ├─ File format fuzzer
│   ├─ KQL parser fuzzer
│   ├─ SQL query fuzzer
│   └─ Run continuously in CI
└─ Coverage: Aim for > 90% code coverage

RISK: No Security Audit Trail for Code Changes
├─ Current: Git log, but no signature verification
├─ Impact: Cannot prove who made what change
├─ Fix: Require signed commits
│   ├─ git config commit.gpgsign true
│   ├─ Verify PRs signed by known developers
│   └─ Audit log of all changes
└─ Compliance: Required for SOC2, ISO27001
```

---

### **VII. SECURITY-SENSITIVE AREAS**

#### **A. Encryption & Key Management**

```
⚠️ CRITICAL: AES-256-GCM Implementation
├─ Location: crates/kcm-security/src/encryption.rs
├─ Risk: 
│   ├─ IV/nonce reuse = breaks GCM security
│   ├─ Key material in memory (not wiped)
│   ├─ No authenticated encryption verification
│   └─ AAD (additional auth data) usage unclear
├─ Must Check:
│   ├─ Each encryption uses unique nonce
│   ├─ Nonce never repeats for same key
│   ├─ AAD includes context/ownership info
│   └─ Key rotation strategy
└─ Action: Third-party crypto audit required

⚠️ CRITICAL: CSPRNG Usage (getrandom 0.2)
├─ Risk: Weak randomness = predictable keys/nonces
├─ Check: 
│   ├─ Used for all cryptographic random generation
│   ├─ No mixing with weak PRNGs
│   └─ Proper seeding on all platforms
└─ Test: Compare output to NIST test suite

⚠️ HIGH: Key Storage in Memory
├─ Risk: Keys visible in process memory
├─ Scenario: 
│   ├─ Attacker dumps memory = key stolen
│   ├─ Coredump includes keys
│   ├─ Key in register visible to side-channel
└─ Mitigation: 
│   ├─ Zero memory after use (memzero)
│   ├─ No core dumps in production
│   └─ Consider encrypted key storage
```

---

#### **B. RBAC Implementation**

```
⚠️ CRITICAL: Permission Check in Every Operation
├─ Risk: Single missed check = total privilege escalation
├─ Must Verify:
│   ├─ read(fact) → check READ permission
│   ├─ insert(fact) → check WRITE permission
│   ├─ delete(fact) → check DELETE permission
│   ├─ execute(rule) → check EXECUTE permission
│   └─ admin_op → check ADMIN permission
├─ Code Review: Grep for "insert_fact" count matches "check_write" count
└─ Test: Each permission level test deny + allow

⚠️ HIGH: Context-Scoped Access Control
├─ Risk: User can access facts in wrong context
├─ Scenario:
│   ├─ User A has READ in context "sales"
│   ├─ Tries to read fact in context "engineering"
│   ├─ Permission check misses context scope
│   └─ Data breach
├─ Control: 
│   ├─ Each fact tagged with ContextID
│   ├─ User permissions per (context, operation) pair
│   ├─ Query filter by allowed contexts
│   └─ No cross-context visibility
└─ Verification: Audit that queries filtered by context

⚠️ CRITICAL: Permission Inheritance/Delegation
├─ Risk: Unclear inheritance rules = privilege escalation
├─ Questions:
│   ├─ If user has WRITE in parent context, scoped to children?
│   ├─ Can ADMIN grant EXECUTE without ADMIN being required?
│   ├─ Does privilege decay with delegation?
├─ Action: 
│   ├─ Write formal permission model (Access Matrix)
│   ├─ Document inheritance rules explicitly
│   └─ Add permission constraint tests
```

---

#### **C. Audit Logging**

```
⚠️ CRITICAL: Tamper-Proof Audit Log
├─ Risk: Attacker deletes audit trail of crime
├─ Requirements:
│   ├─ Write-once: Cannot delete/modify old entries
│   ├─ Append-only: Only new entries added
│   ├─ Hash chain: Each entry hashes previous
│   ├─ Remote archival: Copy off-system
│   └─ Integrity verification: Detect tampering
├─ Current Implementation: Likely mutable Vec - NOT tamper-proof!
└─ Fix: Implement append-only transaction log

⚠️ HIGH: Comprehensive Audit Coverage
├─ Must Log:
│   ├─ Every fact insert/update/delete + who + when
│   ├─ Every permission check (success & failure)
│   ├─ Every login/logout
│   ├─ Every key operation (rotation, deletion)
│   ├─ Every export/data-subject operation
│   └─ Every integrity check failure
├─ Format: Structured JSON with timestamp, user, action, result
└─ Retention: 7 years minimum (compliance)

⚠️ CRITICAL: Audit Log Capacity Management
├─ Current: 100K events limit (then what?)
├─ Risks:
│   ├─ Oldest events overwritten (lose history)
│   ├─ Compliance violation (can't prove audit trail)
│   ├─ Attacker triggers overflow to cover tracks
├─ Solution:
│   ├─ Rotating log files (daily/weekly rollover)
│   ├─ Compression of old logs
│   ├─ Remote archival to S3/blob storage
│   └─ Monitoring: Alert on 90% capacity
└─ Recovery: Never delete live logs, archive only
```

---

### **VIII. CONFIGURATION & TUNING RISKS**

```
⚠️ DANGEROUS DEFAULTS

1. Compression Codec Per Column
   ├─ No guidance on when to use which codec
   ├─ Naive choice = poor performance/ratio
   ├─ Recommendation: Provide codec selection guide
   └─ Example:
       ├─ Predicate → RLE (high repetition)
       ├─ Confidence → Gorilla (float compression)
       ├─ Timestamp → Delta (monotonic)
       └─ Subject/Object → Dictionary (string cardinality)

2. Max Iterations in Inference
   ├─ Default 1000? No timeout mentioned
   ├─ Risk: Pathological rules run 1000 iterations = hang
   ├─ Fix: Combine iteration limit + time budget
   └─ Recommendation: max(100 iterations, 60 seconds)

3. Cache Size / Memory Limits
   ├─ Unbounded column memory allocation
   ├─ Risk: 1TB knowledge graph = OOM
   ├─ Recommendation: 
   │   ├─ Default 1GB cache
   │   ├─ LRU eviction policy
   │   └─ Monitor + alert at 80% usage

4. WAL Batch Size
   ├─ Not mentioned - probably per-operation fsync
   ├─ Performance killer: 50K facts/sec target
   ├─ Recommendation: 
   │   ├─ Batch 1000 writes
   │   ├─ fsync every 100ms
   │   └─ Measure throughput impact

5. Thread Pool Size (Rayon)
   ├─ Default: # of CPU cores
   ├─ Recommendation: Make configurable
   │   ├─ Respect RAYON_NUM_THREADS env var
   │   ├─ Allow override per database instance
   │   └─ Document tradeoff (parallelism vs GC)
```

---

### **IX. PRIORITY CHECKLIST FOR DEVELOPMENT**

#### **BLOCKING (Must Fix Before v1.0)**

- [ ] Buffer overflow in decompression (CRITICAL)
- [ ] Confidence calculus validation (CRITICAL)
- [ ] PredicateID escalate to u16/u32 (CRITICAL)
- [ ] Crash recovery test (CRITICAL)
- [ ] Benchmark baseline collection (CRITICAL)
- [ ] Optimizer cost model definition (CRITICAL)
- [ ] Inference timeout mechanism (CRITICAL)
- [ ] GDPR deletion atomicity (CRITICAL)

#### **HIGH PRIORITY (Before v0.5)**

- [ ] Health check endpoint
- [ ] Graceful shutdown handler
- [ ] Audit log tamper-proofing
- [ ] Compression overflow checks
- [ ] Distributed mode 2PC tests
- [ ] Query plan caching
- [ ] Multi-version concurrency (MVCC) planning

#### **MEDIUM PRIORITY (Before v0.8)**

- [ ] Performance profiling hooks
- [ ] Prometheus metrics export
- [ ] Kubernetes livenessProbe/readinessProbe
- [ ] Fuzzing infrastructure
- [ ] Dictionary saturation handling
- [ ] Memory budget enforcement
- [ ] Connection pooling

#### **NICE TO HAVE (Future)**

- [ ] Window functions
- [ ] Learned indexes
- [ ] Adaptive compression
- [ ] Query result caching
- [ ] Distributed execution planner

---

### **X. TESTING REQUIREMENTS**

```
MUST HAVE TESTS:

1. Unsafe Code Paths
   ├─ DenseVec allocation with max capacity
   ├─ Alignment boundaries for T
   ├─ Clone under memory pressure
   ├─ Concurrent access (ThreadSanitizer)
   └─ Frequency: Every commit

2. Compression/Decompression
   ├─ Roundtrip: Compress(x) → Decompress() == x
   ├─ Size mismatch detection
   ├─ Corrupted data detection
   ├─ Each codec: Zstd, LZ4, RLE
   └─ Property: For all data D: decompress(compress(D)) == D

3. Confidence Calculus
   ├─ multiply(0.5, 0.5) == 0.25
   ├─ combine_or(0.5, 0.5) == 0.75
   ├─ Boundary: multiply(1.0, 1.0) == 1.0
   ├─ Precision: Very small numbers < 1e-10
   ├─ NaN handling: multiply(0.5, NaN) == error
   └─ Property: For all c1, c2: 0 <= multiply(c1, c2) <= 1

4. Inference
   ├─ Fixed point: No new facts after N iterations
   ├─ Timeout: Inference stops after 60 seconds
   ├─ Duplicate prevention: Same fact not inferred twice
   ├─ Negation: NOT operator semantics clear
   └─ Stress: 10K facts + 100 rules + 10 iterations

5. Crash Recovery
   ├─ Kill -9 during column flush → recoverable
   ├─ Kill -9 during WAL write → no data loss
   ├─ Partial backup file → restore detects corruption
   ├─ WAL replay: All writes before crash recovered
   └─ Frequency: Weekly full recovery test

6. Concurrency
   ├─ 100 reader threads + 1 writer thread
   ├─ No data races (ThreadSanitizer)
   ├─ No deadlocks (static analysis or timeout)
   ├─ No stale reads (consistent snapshots)
   └─ Latency: Readers not starved > 10 seconds

7. Security
   ├─ RBAC: Each permission level properly enforced
   ├─ Encryption: IV/nonce never reused
   ├─ Audit: Every operation logged
   ├─ GDPR: Right-to-delete actually deletes
   └─ Audit trail: Cannot be modified after creation
```

---

### **XI. DOCUMENTATION REQUIREMENTS**

```
CRITICAL DOCS TO WRITE:

1. KQL Grammar
   ├─ EBNF formal specification
   ├─ Examples for each clause
   ├─ Error cases + error messages
   └─ Version: Update with language changes

2. Cost Model
   ├─ CPU cost formula per operator
   ├─ I/O cost formula per operator
   ├─ Memory cost formula
   ├─ Cardinality estimation algorithm
   └─ Example: Estimate cost of SELECT * FROM facts WHERE confidence > 0.5

3. Compression Codec Selection Guide
   ├─ When to use RLE
   ├─ When to use Dictionary
   ├─ When to use Delta
   ├─ When to use Gorilla
   └─ Ratios achieved on sample data

4. GDPR Implementation Details
   ├─ Right-to-deletion: Exactly how facts deleted
   ├─ Data portability: Format of exported data
   ├─ Consent management: Enforcement mechanisms
   ├─ Data classification: What each tier means
   └─ Retention: When data automatically purged

5. Recovery Procedures
   ├─ Normal startup
   ├─ Recovery from WAL after crash
   ├─ Recovery from corruption
   ├─ Restore from backup
   ├─ Point-in-time recovery
   └─ Test procedures with examples

6. Performance Tuning
   ├─ Cache sizing
   ├─ Thread pool configuration
   ├─ Compression level tuning
   ├─ Index selection strategy
   ├─ Query plan hints
   └─ Benchmark results on reference hardware

7. Deployment Guide
   ├─ Docker/Kubernetes setup
   ├─ Health check configuration
   ├─ Backup strategy
   ├─ Monitoring setup
   ├─ Troubleshooting guide
   └─ Scaling guidelines (single node limits)
```

---

**SUMMARY:** KCM has ~50 documented gaps + ~40 sensitive code areas that need careful handling. The TIER 1 blockers must be fixed immediately before any serious deployment. The confidence calculus, unsafe memory operations, and crash recovery are the most critical areas to audit carefully.
