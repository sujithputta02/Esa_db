# Day 6: RocksDB Integration - Implementation Guide

**Focus Areas:** RocksDB Rust Bindings, Column Families, Bloom Filters, Backup/Restore, POC Integration

---

## 1. RocksDB Rust Bindings Overview

### What is rust-rocksdb?
`rust-rocksdb` is the official Rust wrapper for RocksDB, providing safe and idiomatic Rust APIs for the C++ RocksDB library.

### Why rust-rocksdb for ESA?
✅ **Memory Safety**: Rust's ownership prevents memory leaks  
✅ **Zero-cost Abstractions**: Performance of C++ with safety of Rust  
✅ **Active Development**: Well-maintained with regular updates  
✅ **Feature Complete**: Supports all major RocksDB features  
✅ **Multi-threaded**: Built-in support for concurrent access  

### Installation
```toml
[dependencies]
rocksdb = { version = "0.22", features = ["multi-threaded-cf"] }
```

**Features Explained:**
- `multi-threaded-cf`: Enable multi-threaded column family support
- `snappy`: Snappy compression (enabled by default)
- `lz4`: LZ4 compression
- `zstd`: Zstandard compression
- `bzip2`: Bzip2 compression

---

## 2. Column Families in RocksDB

### What are Column Families?
Column Families are like separate "logical databases" within a single RocksDB instance. They share the same WAL but have independent:
- MemTables
- SSTables
- Compaction settings
- Bloom filters
- Cache settings

### ESA Column Family Design

```rust
// ESA Column Families
enum EsaColumnFamily {
    Entities,      // Entity state data
    Metadata,      // Entity metadata (policies, hints)
    RaftLog,       // Raft consensus log
    Snapshots,     // Cluster snapshots
    Metrics,       // Performance metrics
}
```

### Column Family Benefits

**1. Logical Separation**
```
┌─────────────────────────────────────┐
│         RocksDB Instance            │
├─────────────────────────────────────┤
│  CF: entities                       │
│    - entity:123 → state_data        │
│    - entity:456 → state_data        │
├─────────────────────────────────────┤
│  CF: metadata                       │
│    - entity:123 → policies          │
│    - entity:456 → policies          │
├─────────────────────────────────────┤
│  CF: raft_log                       │
│    - log:1 → entry_1                │
│    - log:2 → entry_2                │
└─────────────────────────────────────┘
```

**2. Independent Tuning**
```rust
// Entities CF: Optimize for write throughput
let mut entities_opts = Options::default();
entities_opts.set_write_buffer_size(128 * 1024 * 1024); // 128MB
entities_opts.set_max_write_buffer_number(4);

// Raft Log CF: Optimize for sequential writes
let mut raft_opts = Options::default();
raft_opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
raft_opts.set_disable_auto_compactions(false);

// Metadata CF: Small, optimize for reads
let mut metadata_opts = Options::default();
metadata_opts.set_write_buffer_size(16 * 1024 * 1024); // 16MB
```

---

## 3. Bloom Filters

### What are Bloom Filters?
A Bloom filter is a space-efficient probabilistic data structure that tests whether an element is a member of a set.

**Bloom Filter Properties:**
- **False Positives**: Possible ("maybe in set")
- **False Negatives**: Impossible ("definitely not in set")
- **Space Efficient**: Uses bits, not bytes

### How Bloom Filters Work

```
Key: "entity:123"
         ↓
   Hash Functions (3 different)
         ↓
    Bit Positions: [5, 42, 89]
         ↓
   Set bits to 1 in bitmap

Bloom Filter Bitmap (100 bits):
[0,0,0,0,0,1,0,0,...,1,0,...,1,0,0]
         ↑        ↑        ↑
       bit 5    bit 42   bit 89
```

**Lookup Process:**
```rust
fn may_contain(key: &str) -> bool {
    let positions = hash_key(key); // [5, 42, 89]
    
    // Check if ALL bits are set
    for pos in positions {
        if bitmap[pos] == 0 {
            return false; // Definitely NOT in set!
        }
    }
    
    return true; // Maybe in set (could be false positive)
}
```

### Bloom Filters in RocksDB

**Impact on Read Performance:**
```
Without Bloom Filter:
Read request → Check SSTable-1 (disk I/O) → Not found
             → Check SSTable-2 (disk I/O) → Not found
             → Check SSTable-3 (disk I/O) → Not found
             → Check SSTable-4 (disk I/O) → Found!
Total: 4 disk I/Os

With Bloom Filter:
Read request → Bloom says NO to SSTable-1 → Skip!
             → Bloom says NO to SSTable-2 → Skip!
             → Bloom says NO to SSTable-3 → Skip!
             → Bloom says YES to SSTable-4 → Read (disk I/O) → Found!
Total: 1 disk I/O (75% reduction!)
```

**Configuration:**
```rust
use rocksdb::{Options, BlockBasedOptions};

let mut opts = Options::default();
let mut block_opts = BlockBasedOptions::default();

// Bloom filter with 10 bits per key (1% false positive rate)
block_opts.set_bloom_filter(10.0, false);

// Prefix bloom filter (for prefix scans)
block_opts.set_bloom_filter(10.0, true); // prefix mode

opts.set_block_based_table_factory(&block_opts);
```

**Bloom Filter Types:**

1. **Full Key Bloom Filter**
   - Tests complete keys
   - Best for point lookups: `get("entity:123")`

2. **Prefix Bloom Filter**
   - Tests key prefixes
   - Best for range scans: `scan("entity:")`
   - Perfect for ESA entity lookups!

**ESA Configuration:**
```rust
// Enable prefix bloom for entity lookups
opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(7));
// Extracts "entity:" prefix (7 bytes)

block_opts.set_bloom_filter(10.0, true); // prefix mode enabled
```

---

## 4. Backup and Restore Strategies

### RocksDB Backup Methods

#### Method 1: Checkpoint (Snapshot)
**Best for:** Quick local backups, hot backups

```rust
use rocksdb::{DB, checkpoint::Checkpoint};

// Create checkpoint (hard links, instant)
let checkpoint = Checkpoint::new(&db)?;
checkpoint.create_checkpoint("/backup/snapshot-001")?;

// Checkpoint is a full, consistent snapshot
// Uses hard links (no data copy, instant)
// Database remains operational during checkpoint
```

**Advantages:**
- ✅ Instant (uses hard links)
- ✅ No downtime
- ✅ Consistent snapshot
- ✅ Space efficient

**Disadvantages:**
- ❌ Same filesystem only
- ❌ Not for off-site backups

#### Method 2: Backup Engine
**Best for:** Incremental backups, off-site backups

```rust
use rocksdb::{DB, BackupEngine, BackupEngineOptions};

// Create backup engine
let backup_path = "/backup/rocksdb-backups";
let mut backup_opts = BackupEngineOptions::default();
backup_opts.set_backup_dir(backup_path);

let mut backup_engine = BackupEngine::open(&backup_opts, &env)?;

// Create backup (copies data)
backup_engine.create_new_backup(&db)?;

// Create incremental backup (only new/changed files)
backup_engine.create_new_backup(&db)?; // 2nd backup is incremental!

// List backups
let backup_info = backup_engine.get_backup_info();
for info in backup_info {
    println!("Backup {}: {} bytes", info.backup_id, info.size);
}

// Restore from backup
let restore_path = "/data/restored-db";
backup_engine.restore_from_backup(restore_path, restore_path, 1)?;
```

**Advantages:**
- ✅ Incremental backups
- ✅ Off-site capable
- ✅ Built-in retention policies
- ✅ Verify backup integrity

**Disadvantages:**
- ❌ Slower than checkpoint
- ❌ Requires more disk space initially

#### Method 3: Manual SSTable Copy
**Best for:** Custom backup solutions

```rust
// 1. Disable auto-compaction
db.set_options(&[("disable_auto_compactions", "true")])?;

// 2. Flush memtables
db.flush()?;

// 3. Copy SST files manually
std::fs::copy("/data/db/*.sst", "/backup/")?;

// 4. Copy MANIFEST and CURRENT
std::fs::copy("/data/db/MANIFEST-*", "/backup/")?;
std::fs::copy("/data/db/CURRENT", "/backup/")?;

// 5. Re-enable compaction
db.set_options(&[("disable_auto_compactions", "false")])?;
```

### ESA Backup Strategy

```rust
pub struct EsaBackupManager {
    db: Arc<DB>,
    backup_engine: BackupEngine,
    checkpoint_path: PathBuf,
}

impl EsaBackupManager {
    // Hot backup using checkpoint (instant, for local snapshots)
    pub fn create_hot_snapshot(&self) -> Result<String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let snapshot_path = self.checkpoint_path.join(format!("snap-{}", timestamp));
        
        let checkpoint = Checkpoint::new(&self.db)?;
        checkpoint.create_checkpoint(&snapshot_path)?;
        
        Ok(format!("Created snapshot: {:?}", snapshot_path))
    }
    
    // Incremental backup (for durability, retention)
    pub fn create_incremental_backup(&mut self) -> Result<u32> {
        self.backup_engine.create_new_backup(&self.db)?;
        
        let backup_info = self.backup_engine.get_backup_info();
        let latest = backup_info.last().unwrap();
        
        Ok(latest.backup_id)
    }
    
    // Restore from backup
    pub fn restore_from_backup(&mut self, backup_id: u32, restore_path: &str) -> Result<()> {
        self.backup_engine.restore_from_backup(
            restore_path,
            restore_path,
            backup_id
        )?;
        
        Ok(())
    }
    
    // Retention policy: Keep last N backups
    pub fn apply_retention_policy(&mut self, keep_last: usize) -> Result<()> {
        let backup_info = self.backup_engine.get_backup_info();
        
        if backup_info.len() > keep_last {
            let to_delete = backup_info.len() - keep_last;
            
            for i in 0..to_delete {
                let backup_id = backup_info[i].backup_id;
                self.backup_engine.purge_old_backups(1)?;
            }
        }
        
        Ok(())
    }
}
```

---

## 5. Proof-of-Concept: ESA Storage Layer

### Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│              ESA Storage Layer                          │
├─────────────────────────────────────────────────────────┤
│  EsaStorage                                             │
│    ├─ RocksDB Instance                                 │
│    ├─ Column Families                                   │
│    │   ├─ entities    (versioned entity state)         │
│    │   ├─ metadata    (policies, locality hints)       │
│    │   ├─ raft_log    (consensus log)                  │
│    │   └─ snapshots   (cluster snapshots)              │
│    ├─ Backup Manager                                    │
│    └─ Configuration                                     │
└─────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Versioned Keys for MVCC**
   ```rust
   // Key format: {entity_id}:{timestamp}
   let key = format!("entity:{}:{}", entity_id, timestamp);
   ```

2. **Column Family Separation**
   - Entities: High write throughput
   - Metadata: Small, read-heavy
   - Raft Log: Sequential writes
   - Snapshots: Infrequent, large writes

3. **Bloom Filters**
   - Prefix bloom for entity: prefix
   - 10 bits per key (1% false positive rate)

4. **Compaction Strategy**
   - Leveled compaction for entities (balanced)
   - Universal compaction for raft_log (write-optimized)

5. **Backup Strategy**
   - Checkpoint every hour (local snapshots)
   - Incremental backup every 6 hours (durability)
   - Retain last 7 days of backups

---

## 6. Implementation Code Structure

### File Organization
```
src/storage/
├── mod.rs           # Public API
├── engine.rs        # Core RocksDB wrapper
├── config.rs        # Configuration
├── backup.rs        # Backup/restore logic
├── column_family.rs # CF management
└── mvcc.rs          # MVCC helpers
```

### Key Types
```rust
// Entity key with timestamp (MVCC)
pub struct EntityKey {
    pub entity_id: String,
    pub timestamp: u64,
}

// Storage configuration
pub struct StorageConfig {
    pub path: PathBuf,
    pub enable_bloom_filters: bool,
    pub write_buffer_size: usize,
    pub max_open_files: i32,
}

// Storage statistics
pub struct StorageStats {
    pub num_keys: u64,
    pub total_size: u64,
    pub compaction_pending: bool,
}
```

---

## 7. Performance Tuning

### Memory Settings
```rust
// Block cache (shared across all CFs)
let cache = rocksdb::Cache::new_lru_cache(512 * 1024 * 1024)?; // 512MB

let mut block_opts = BlockBasedOptions::default();
block_opts.set_block_cache(&cache);
block_opts.set_block_size(16 * 1024); // 16KB blocks
```

### Write Buffer
```rust
// Write buffer per CF
opts.set_write_buffer_size(128 * 1024 * 1024); // 128MB
opts.set_max_write_buffer_number(4); // Up to 4 memtables
opts.set_min_write_buffer_number_to_merge(2); // Merge 2 before flush
```

### Compaction
```rust
// Compaction threads
opts.set_max_background_jobs(4); // 4 background threads

// Level 0 compaction trigger
opts.set_level_zero_file_num_compaction_trigger(4);
opts.set_level_zero_slowdown_writes_trigger(20);
opts.set_level_zero_stop_writes_trigger(36);
```

### Bloom Filters
```rust
// 10 bits per key = ~1% false positive rate
// 15 bits per key = ~0.1% false positive rate
block_opts.set_bloom_filter(10.0, false);
```

---

## 8. Best Practices for ESA

### ✅ Do's

1. **Use Column Families**
   - Separate data types into different CFs
   - Tune each CF independently

2. **Enable Bloom Filters**
   - Use prefix bloom for entity lookups
   - Reduces disk I/O significantly

3. **Batch Writes**
   ```rust
   let mut batch = WriteBatch::default();
   batch.put("key1", "value1");
   batch.put("key2", "value2");
   db.write(batch)?; // Single atomic write
   ```

4. **Use Snapshots for Consistent Reads**
   ```rust
   let snapshot = db.snapshot();
   let value = snapshot.get("key")?;
   ```

5. **Monitor Performance**
   ```rust
   let stats = db.property_value("rocksdb.stats")?;
   println!("RocksDB stats: {}", stats.unwrap());
   ```

### ❌ Don'ts

1. **Don't Open DB Multiple Times**
   - Use Arc<DB> for sharing
   - Single instance per process

2. **Don't Forget to Flush on Shutdown**
   ```rust
   db.flush()?;
   drop(db); // Close properly
   ```

3. **Don't Use Sync Writes for Everything**
   - Sync writes are slow (fsync)
   - Use for critical data only

4. **Don't Ignore Compaction**
   - Monitor compaction backlog
   - Tune compaction settings

---

## 9. Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_basic_read_write() {
        let tmp = TempDir::new().unwrap();
        let db = DB::open_default(tmp.path()).unwrap();
        
        db.put(b"key", b"value").unwrap();
        let result = db.get(b"key").unwrap().unwrap();
        
        assert_eq!(result.as_ref(), b"value");
    }
    
    #[test]
    fn test_column_families() {
        // Test CF creation and isolation
    }
    
    #[test]
    fn test_mvcc_versioning() {
        // Test versioned key reads
    }
}
```

### Integration Tests
- Multi-threaded concurrent access
- Column family isolation
- Backup and restore
- Crash recovery (simulate crash)

### Performance Tests
- Write throughput benchmarks
- Read latency benchmarks
- Compaction impact tests
- Memory usage profiling

---

## 10. Key Takeaways

### RocksDB in ESA
✅ **High-performance embedded KV store**  
✅ **Column families for logical separation**  
✅ **Bloom filters reduce read amplification**  
✅ **Multiple backup strategies available**  
✅ **Battle-tested in production systems**  

### ESA Storage Design
- **Entities CF**: Versioned entity state (MVCC)
- **Metadata CF**: Policies and hints
- **Raft Log CF**: Consensus log entries
- **Snapshots CF**: Cluster snapshots

### Performance Optimizations
- Prefix bloom filters for entity lookups
- Batched writes for atomicity
- Tuned compaction settings per CF
- Shared block cache across CFs

---

## Resources

### Documentation
- [rust-rocksdb docs](https://docs.rs/rocksdb/)
- [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki)
- [RocksDB Tuning Guide](https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide)

### Examples
- [rust-rocksdb examples](https://github.com/rust-rocksdb/rust-rocksdb/tree/master/tests)

### Books
- "Database Internals" by Alex Petrov

---

**Day 6 Complete! Tomorrow (Day 7): Week 1 review and planning.**
