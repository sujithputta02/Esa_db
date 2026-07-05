# Day 5: Storage Systems Deep Dive - Easy-to-Understand Notes

**Focus Areas:** LSM Trees, RocksDB, MVCC, Write-Ahead Logging (WAL)

---

## 1. LSM Tree (Log-Structured Merge Tree) Architecture

### What is an LSM Tree?
LSM tree is a data structure optimized for **write-heavy workloads**. Instead of updating data in-place (like B-trees), it writes everything sequentially to logs and merges them later.

### Why LSM Trees for ESA?
- **High write throughput**: Perfect for entity state updates
- **Sequential writes**: Much faster than random disk writes
- **Compaction**: Old versions are cleaned up automatically
- **Crash recovery**: Write-ahead log ensures durability

### LSM Tree Components

#### 1. **MemTable** (In-Memory Buffer)
```
┌─────────────────────────────────────┐
│         MemTable (RAM)              │
│  ┌─────────────────────────────┐   │
│  │  Sorted Map (SkipList/Tree) │   │
│  │  key1 -> value1             │   │
│  │  key2 -> value2             │   │
│  │  key3 -> value3             │   │
│  └─────────────────────────────┘   │
│  Size: ~4MB - 64MB (configurable)  │
└─────────────────────────────────────┘
```

**How it works:**
- All writes go here first (in RAM, super fast!)
- Sorted by key automatically
- When full → flushed to disk as SSTable
- Concurrent with new MemTable

**ESA Usage:**
- Entity state updates written to MemTable first
- Fast reads from recent writes
- Batch writes improve throughput

#### 2. **SSTable** (Sorted String Table - On Disk)
```
Disk Storage Layout:

Level 0: [SSTable-1] [SSTable-2] [SSTable-3]  ← Recent, may overlap
         ↓ Compaction
Level 1: [SSTable-4...................]        ← Merged, no overlap
         ↓ Compaction  
Level 2: [SSTable-5...................................]
         ↓ Compaction
Level 3: [SSTable-6...................................................]
```

**SSTable File Structure:**
```
┌───────────────────────────────────────────┐
│         SSTable File (~2MB)               │
├───────────────────────────────────────────┤
│  Data Block 1:                            │
│    key1=value1, key2=value2, ...          │
├───────────────────────────────────────────┤
│  Data Block 2:                            │
│    key100=value100, key101=value101, ...  │
├───────────────────────────────────────────┤
│  ...                                      │
├───────────────────────────────────────────┤
│  Index Block:                             │
│    Block1 starts at offset 0              │
│    Block2 starts at offset 4096           │
├───────────────────────────────────────────┤
│  Bloom Filter:                            │
│    Probabilistic filter for fast lookup   │
├───────────────────────────────────────────┤
│  Footer: Metadata                         │
└───────────────────────────────────────────┘
```

**Key Properties:**
- **Immutable**: Once written, never modified
- **Sorted**: Keys in order for fast range scans
- **Indexed**: Block index for quick seeks
- **Compressed**: Saves space (Snappy, LZ4, Zstd)

#### 3. **Compaction** (Merging and Cleanup)

**Why Compaction?**
- Remove deleted keys (tombstones)
- Remove old versions of updated keys
- Reduce read amplification
- Reclaim disk space

**Compaction Strategies:**

##### a) **Leveled Compaction** (RocksDB default)
```
Level 0: 4 files (overlapping)    |  Size: ~256MB
         ↓ Compact when 4 files
Level 1: 1 file (no overlap)      |  Size: ~256MB
         ↓ Compact when > 256MB
Level 2: ~10 files                |  Size: ~2.5GB
         ↓ Compact when > 2.5GB
Level 3: ~100 files               |  Size: ~25GB
```

**Pros:**
- Good read performance (few files to check)
- Space efficient (less space amplification)

**Cons:**
- Write amplification (data rewritten multiple times)

##### b) **Tiered Compaction**
```
Tier 0: [File1] [File2] [File3] [File4]
        ↓ Merge all into one
Tier 1: [Merged-File-1]
        ↓ When tier full
Tier 2: [Merged-File-2] [Merged-File-3]
```

**Pros:**
- Lower write amplification
- Better for write-heavy workloads

**Cons:**
- More files to read (higher read amplification)

**ESA Decision:** Leveled compaction (RocksDB default) for balanced read/write performance.

---

## 2. RocksDB Architecture

### What is RocksDB?
RocksDB is a high-performance embedded key-value store based on LSM trees, created by Facebook (Meta).

### Why RocksDB for ESA?
✅ **Embedded**: No separate server process  
✅ **High throughput**: Millions of writes/sec  
✅ **Column families**: Logical isolation of data  
✅ **Atomic batches**: ACID properties  
✅ **Snapshots**: Consistent reads  
✅ **Prefix bloom filters**: Fast entity lookups  
✅ **Battle-tested**: Used by MySQL, CockroachDB, TiDB  

### RocksDB Components

```
┌─────────────────────────────────────────────────────────┐
│                     RocksDB                             │
├─────────────────────────────────────────────────────────┤
│  Write Path:                                            │
│    Write → WAL → MemTable → SSTable                     │
│                                                          │
│  Read Path:                                             │
│    Read → MemTable → Block Cache → SSTable              │
├─────────────────────────────────────────────────────────┤
│  Column Families:                                       │
│    - "entities"    (entity state)                       │
│    - "metadata"    (entity metadata)                    │
│    - "raft_log"    (consensus log)                      │
│    - "snapshots"   (cluster snapshots)                  │
├─────────────────────────────────────────────────────────┤
│  Background Tasks:                                      │
│    - Flush MemTable to L0                               │
│    - Compact L0 to L1                                   │
│    - Compact L1+ levels                                 │
│    - Delete obsolete files                              │
└─────────────────────────────────────────────────────────┘
```

### Column Families in ESA

**What are Column Families?**
Think of them as separate "databases" within one RocksDB instance.

```rust
// ESA Column Family Design
let cf_entities = db.cf_handle("entities").unwrap();
let cf_metadata = db.cf_handle("metadata").unwrap();
let cf_raft_log = db.cf_handle("raft_log").unwrap();

// Different tuning per column family
// entities: fast reads, medium writes
// raft_log: fast sequential writes
// metadata: small, infrequent access
```

**Benefits:**
- **Isolation**: Different data types don't interfere
- **Custom tuning**: Each CF can have different settings
- **Atomic operations**: Batch writes across CFs are atomic
- **Independent compaction**: Compact CFs separately

### RocksDB Write Path

```
1. Write Request
   ↓
2. Write to WAL (durability guarantee)
   ↓
3. Write to MemTable (in memory)
   ↓
4. Return success to client (write complete!)
   ↓
5. Background: Flush MemTable → SSTable when full
   ↓
6. Background: Compact SSTables periodically
```

**Key Insight:** Writes are fast because they're sequential (WAL + MemTable).

### RocksDB Read Path

```
1. Read Request (key="entity:123")
   ↓
2. Check MemTable (newest data)
   ↓ If not found
3. Check Immutable MemTable(s)
   ↓ If not found
4. Check Block Cache (recently read blocks)
   ↓ If not found
5. Check Bloom Filters (is key in this SSTable?)
   ↓ Probably yes
6. Read from SSTable (Level 0, 1, 2, ...)
   ↓
7. Return value
```

**Optimization: Bloom Filters**
```
Bloom Filter: "Is key X in this SSTable?"
- False: Definitely NOT in file (skip file)
- True: Probably in file (check file)
```

Saves 90%+ of disk reads!

---

## 3. MVCC (Multi-Version Concurrency Control)

### What is MVCC?
MVCC keeps multiple versions of data, allowing readers and writers to work without blocking each other.

### Why MVCC for ESA?
- **No read locks**: Readers never block writers
- **Snapshot isolation**: Consistent point-in-time reads
- **Optimistic concurrency**: Better for distributed systems
- **Time-travel queries**: Read old entity states

### MVCC Implementation

#### Versioned Keys
```
Physical Storage (LSM tree):
key="entity:123", timestamp=t1 → value="state_v1"
key="entity:123", timestamp=t2 → value="state_v2"
key="entity:123", timestamp=t3 → value="state_v3"

Logical View at t2:
entity:123 → state_v2

Logical View at t3:
entity:123 → state_v3
```

#### ESA MVCC Design
```rust
// Versioned key format
struct VersionedKey {
    entity_id: String,      // "entity:123"
    timestamp: u64,         // Logical timestamp
}

// Read at specific timestamp
fn read_at_version(entity_id: &str, ts: u64) -> Option<State> {
    // Find latest version <= ts
    let key_start = format!("{}:{}", entity_id, 0);
    let key_end = format!("{}:{}", entity_id, ts);
    
    // Seek to highest key <= ts
    db.get_latest_version(key_start, key_end)
}

// Write new version
fn write_version(entity_id: &str, state: State) {
    let ts = get_current_timestamp();
    let key = format!("{}:{}", entity_id, ts);
    db.put(key, state);
}
```

### MVCC Garbage Collection

**Problem:** Old versions pile up, waste space.

**Solution:** Periodic GC
```rust
// Keep last N versions or versions within time window
fn gc_old_versions(entity_id: &str, keep_versions: usize) {
    let versions = get_all_versions(entity_id);
    
    // Keep only last N versions
    for version in versions.skip(keep_versions) {
        db.delete(version.key);
    }
}
```

**ESA GC Strategy:**
- Keep last 100 versions per entity (configurable)
- Keep versions from last 24 hours
- Run GC during low-load periods

---

## 4. Write-Ahead Log (WAL)

### What is WAL?
WAL is a sequential log file where all writes are recorded **before** being applied to the database.

### Why WAL?
**Durability**: If crash happens, replay WAL to recover lost data.

```
Without WAL:
Write to MemTable → CRASH → Data lost!

With WAL:
Write to WAL → Write to MemTable → CRASH → Replay WAL → Data recovered!
```

### WAL Structure

```
WAL File Format:
┌─────────────────────────────────────┐
│  Record 1:                          │
│    Sequence: 1                      │
│    Type: PUT                        │
│    Key: "entity:123"                │
│    Value: "state_data"              │
│    Checksum: 0xABCD                 │
├─────────────────────────────────────┤
│  Record 2:                          │
│    Sequence: 2                      │
│    Type: DELETE                     │
│    Key: "entity:456"                │
│    Checksum: 0x1234                 │
├─────────────────────────────────────┤
│  ...                                │
└─────────────────────────────────────┘
```

### WAL Lifecycle

```
1. Database Startup
   ↓
2. Replay WAL (recover any uncommitted writes)
   ↓
3. Normal operation: Write to WAL + MemTable
   ↓
4. MemTable full → Flush to SSTable
   ↓
5. Delete old WAL (data now in SSTable)
   ↓
6. Create new WAL
```

### WAL Configuration in ESA

```rust
use rocksdb::{Options, DBWithThreadMode, SingleThreaded};

let mut opts = Options::default();

// WAL settings
opts.set_wal_dir("/data/wal");           // Separate WAL directory
opts.set_wal_size_limit_mb(512);         // Max WAL size
opts.set_wal_ttl_seconds(3600);          // WAL retention
opts.set_manual_wal_flush(false);        // Auto-flush WAL

// Durability levels
opts.set_use_fsync(true);                 // fsync for durability
// or
opts.set_use_fsync(false);                // faster, less durable

let db = DB::open(&opts, "/data/db").unwrap();
```

**ESA WAL Strategy:**
- **Sync mode**: For critical entity state changes (slower but safe)
- **Async mode**: For metrics and non-critical data (faster)

---

## 5. Putting It All Together: ESA Storage Architecture

### ESA Storage Layout

```
┌─────────────────────────────────────────────────────────────┐
│                  ESA Storage Layer                          │
├─────────────────────────────────────────────────────────────┤
│  Column Family: "entities"                                  │
│    - Key: entity_id:timestamp                               │
│    - Value: EntityState (protobuf/bincode)                  │
│    - Settings: High write throughput, bloom filters         │
├─────────────────────────────────────────────────────────────┤
│  Column Family: "metadata"                                  │
│    - Key: entity_id                                         │
│    - Value: EntityMetadata (policies, hints)                │
│    - Settings: Small dataset, fast reads                    │
├─────────────────────────────────────────────────────────────┤
│  Column Family: "raft_log"                                  │
│    - Key: log_index                                         │
│    - Value: RaftEntry                                       │
│    - Settings: Sequential writes, prefix bloom              │
├─────────────────────────────────────────────────────────────┤
│  Column Family: "snapshots"                                 │
│    - Key: snapshot_id                                       │
│    - Value: ClusterSnapshot                                 │
│    - Settings: Infrequent writes, compression               │
└─────────────────────────────────────────────────────────────┘
```

### Example: Entity Write Flow

```rust
// 1. Entity state update
let entity_id = "entity:user:12345";
let new_state = EntityState {
    version: 10,
    data: vec![1, 2, 3],
    metadata: Metadata::default(),
};

// 2. Create versioned key
let timestamp = get_logical_timestamp();
let key = format!("{}:{}", entity_id, timestamp);

// 3. Write batch (atomic across CFs)
let mut batch = WriteBatch::default();

// Write to entities CF
batch.put_cf(cf_entities, key.as_bytes(), serialize(&new_state));

// Update metadata CF
batch.put_cf(cf_metadata, entity_id.as_bytes(), serialize(&metadata));

// 4. Commit batch (writes to WAL + MemTable)
db.write(batch)?;

// Flow:
// WAL: [PUT entity:user:12345:1000 = state] (durable)
// MemTable: entity:user:12345:1000 = state (fast)
// Background: Flush to SSTable when MemTable full
```

### Example: Entity Read with MVCC

```rust
// Read entity at specific timestamp
fn read_entity_at(entity_id: &str, timestamp: u64) -> Option<EntityState> {
    let key_prefix = format!("{}:", entity_id);
    let key_at_ts = format!("{}:{}", entity_id, timestamp);
    
    // Reverse iterator: find latest version <= timestamp
    let mut iter = db.iterator_cf(
        cf_entities,
        IteratorMode::From(key_at_ts.as_bytes(), Direction::Reverse)
    );
    
    while let Some(Ok((key, value))) = iter.next() {
        let key_str = String::from_utf8(key.to_vec()).unwrap();
        
        if key_str.starts_with(&key_prefix) {
            let ts = extract_timestamp(&key_str);
            if ts <= timestamp {
                return Some(deserialize(&value));
            }
        } else {
            break; // Past this entity's keys
        }
    }
    
    None
}

// Read latest version (no timestamp)
fn read_entity_latest(entity_id: &str) -> Option<EntityState> {
    read_entity_at(entity_id, u64::MAX)
}
```

---

## 6. Performance Characteristics

### Write Performance
```
LSM Tree Writes:
- Throughput: 100k - 1M writes/sec (SSD)
- Latency: 0.1 - 1ms (with WAL fsync)
- Latency: < 0.1ms (without fsync)

Factors:
+ Sequential writes (fast)
+ Batching (amortizes fsync cost)
- WAL fsync (durability cost)
- Compaction overhead (background)
```

### Read Performance
```
LSM Tree Reads:
- Best case: 0.01ms (MemTable hit)
- Good case: 0.1ms (Block cache hit)
- Worst case: 1-10ms (SSTable reads across levels)

Optimizations:
+ Bloom filters (skip non-existent keys)
+ Block cache (cache hot data)
+ Prefix bloom (fast prefix scans)
- Read amplification (check multiple levels)
```

### ESA Tuning Targets
- **Entity writes**: 50k-100k/sec per node
- **Entity reads**: 100k-200k/sec per node
- **Raft log writes**: 10k-20k entries/sec
- **Replication lag**: < 100ms

---

## 7. Key Takeaways for ESA

### Why These Technologies?
1. **LSM Trees**: Write-heavy workloads (entity state updates)
2. **RocksDB**: Battle-tested, embedded, column families
3. **MVCC**: Snapshot reads for Raft, time-travel queries
4. **WAL**: Crash recovery, durability guarantees

### Design Decisions
✅ **Leveled compaction**: Balanced read/write performance  
✅ **Column families**: Logical separation (entities, metadata, raft_log)  
✅ **Versioned keys**: MVCC with timestamp in key  
✅ **Bloom filters**: Reduce read amplification  
✅ **WAL fsync**: Enabled for entity state, disabled for metrics  

### Next Steps (Day 6)
- Explore RocksDB Rust bindings (`rust-rocksdb`)
- Create proof-of-concept entity storage
- Test write/read performance
- Implement basic MVCC with versioned keys

---

## 8. Visual Summary

```
┌─────────────────────────────────────────────────────────────┐
│                    ESA Storage Stack                        │
├─────────────────────────────────────────────────────────────┤
│  Application Layer (ESA Runtime)                            │
│    ↕                                                         │
│  RocksDB API (Column Families)                              │
│    ↕                                                         │
│  LSM Tree (MemTable + SSTables)                             │
│    ↕                                                         │
│  WAL (Write-Ahead Log)                                      │
│    ↕                                                         │
│  File System (SSDs)                                         │
└─────────────────────────────────────────────────────────────┘

Write Path: App → WAL → MemTable → (flush) → SSTable → (compact) → Merged SSTable
Read Path:  App → MemTable → Block Cache → Bloom Filter → SSTable → Disk
```

---

## Resources for Deeper Learning

### Papers
- ["The Log-Structured Merge-Tree (LSM-Tree)" - O'Neil et al.](http://www.benstopford.com/2015/02/14/log-structured-merge-trees/)
- ["RocksDB: Evolution of Development Priorities" - Facebook](https://github.com/facebook/rocksdb/wiki)

### Documentation
- [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki)
- [rust-rocksdb docs](https://docs.rs/rocksdb/)

### Videos
- [CMU Database Course - LSM Trees](https://www.youtube.com/watch?v=I6jB0nM9SKU)
- [Understanding LSM Trees](https://www.youtube.com/watch?v=F_dIb1fIKSo)

---

**Day 5 Complete! Tomorrow (Day 6): Hands-on RocksDB implementation and testing.**
