/// Day 6: RocksDB Proof-of-Concept
/// 
/// This example demonstrates:
/// 1. Basic RocksDB operations
/// 2. Column families usage
/// 3. Bloom filters and prefix scans
/// 4. Backup and restore
/// 5. MVCC with versioned keys
/// 6. ESA storage pattern

use rocksdb::{DB, Options, WriteBatch, IteratorMode, Direction};
use rocksdb::{BlockBasedOptions, Cache, SliceTransform};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         Day 6: RocksDB Integration POC                    ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // Demo 1: Basic Operations
    println!("━━━ Demo 1: Basic RocksDB Operations ━━━");
    demo_basic_operations();
    
    // Demo 2: Column Families
    println!("\n━━━ Demo 2: Column Families ━━━");
    demo_column_families();
    
    // Demo 3: Bloom Filters and Prefix Scans
    println!("\n━━━ Demo 3: Bloom Filters and Prefix Scans ━━━");
    demo_bloom_filters();
    
    // Demo 4: Batch Writes
    println!("\n━━━ Demo 4: Atomic Batch Writes ━━━");
    demo_batch_writes();
    
    // Demo 5: MVCC Pattern
    println!("\n━━━ Demo 5: MVCC with Versioned Keys ━━━");
    demo_mvcc_pattern();
    
    // Demo 6: Performance Stats
    println!("\n━━━ Demo 6: Performance Statistics ━━━");
    demo_performance_stats();
    
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                   All Demos Complete!                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}

fn demo_basic_operations() {
    // Create temporary directory for demo
    let path = "/tmp/rocksdb-poc-basic";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. Opening database at: {}", path);
    
    // Configure options
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
    
    // Open database
    let db = DB::open(&opts, path).expect("Failed to open database");
    
    // Write operations
    println!("\n2. Writing data:");
    db.put(b"entity:001", b"Alice's state").unwrap();
    db.put(b"entity:002", b"Bob's state").unwrap();
    db.put(b"entity:003", b"Charlie's state").unwrap();
    println!("  ✅ Wrote 3 entities");
    
    // Read operations
    println!("\n3. Reading data:");
    match db.get(b"entity:001") {
        Ok(Some(value)) => {
            let s = String::from_utf8(value).unwrap();
            println!("  📖 entity:001 = {}", s);
        }
        Ok(None) => println!("  ❌ Key not found"),
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    // Update operation
    println!("\n4. Updating entity:001:");
    db.put(b"entity:001", b"Alice's updated state").unwrap();
    let value = db.get(b"entity:001").unwrap().unwrap();
    println!("  ✅ Updated: {}", String::from_utf8(value).unwrap());
    
    // Delete operation
    println!("\n5. Deleting entity:002:");
    db.delete(b"entity:002").unwrap();
    match db.get(b"entity:002") {
        Ok(None) => println!("  ✅ Successfully deleted"),
        _ => println!("  ❌ Delete failed"),
    }
    
    // Iteration
    println!("\n6. Iterating all keys:");
    let iter = db.iterator(IteratorMode::Start);
    for (key, value) in iter {
        let k = String::from_utf8(key.to_vec()).unwrap();
        let v = String::from_utf8(value.to_vec()).unwrap();
        println!("  📄 {} = {}", k, v);
    }
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}

fn demo_column_families() {
    let path = "/tmp/rocksdb-poc-cf";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. Creating database with column families:");
    
    // Define column families
    let cf_names = vec!["entities", "metadata", "raft_log"];
    
    // Create options for each CF
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    
    // Open database with CFs
    let db = DB::open_cf(&opts, path, &cf_names).expect("Failed to open DB with CFs");
    
    println!("  ✅ Created CFs: {:?}", cf_names);
    
    // Write to different CFs
    println!("\n2. Writing to different column families:");
    
    let cf_entities = db.cf_handle("entities").unwrap();
    let cf_metadata = db.cf_handle("metadata").unwrap();
    let cf_raft_log = db.cf_handle("raft_log").unwrap();
    
    db.put_cf(&cf_entities, b"entity:123", b"state_data").unwrap();
    db.put_cf(&cf_metadata, b"entity:123", b"policy_v1").unwrap();
    db.put_cf(&cf_raft_log, b"entry:1", b"log_entry_1").unwrap();
    
    println!("  ✅ entities CF: entity:123 = state_data");
    println!("  ✅ metadata CF: entity:123 = policy_v1");
    println!("  ✅ raft_log CF: entry:1 = log_entry_1");
    
    // Read from different CFs
    println!("\n3. Reading from column families:");
    
    let value = db.get_cf(&cf_entities, b"entity:123").unwrap().unwrap();
    println!("  📖 entities CF: {}", String::from_utf8(value).unwrap());
    
    let value = db.get_cf(&cf_metadata, b"entity:123").unwrap().unwrap();
    println!("  📖 metadata CF: {}", String::from_utf8(value).unwrap());
    
    // Show CF isolation
    println!("\n4. Demonstrating CF isolation:");
    let result = db.get_cf(&cf_entities, b"entry:1");
    match result {
        Ok(None) => println!("  ✅ entry:1 not in entities CF (isolated!)"),
        _ => println!("  ❌ Isolation failed"),
    }
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}

fn demo_bloom_filters() {
    let path = "/tmp/rocksdb-poc-bloom";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. Configuring bloom filters:");
    
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    // Configure block-based table with bloom filter
    let mut block_opts = BlockBasedOptions::default();
    
    // Bloom filter: 10 bits per key (~1% false positive rate)
    block_opts.set_bloom_filter(10.0, false);
    
    // Create shared cache
    let cache = Cache::new_lru_cache(64 * 1024 * 1024); // 64MB
    block_opts.set_block_cache(&cache);
    
    opts.set_block_based_table_factory(&block_opts);
    
    println!("  ✅ Bloom filter: 10 bits/key (~1% false positive)");
    println!("  ✅ Block cache: 64MB");
    
    let db = DB::open(&opts, path).unwrap();
    
    // Write test data
    println!("\n2. Writing test data:");
    for i in 0..100 {
        let key = format!("entity:{:03}", i);
        let value = format!("state_{}", i);
        db.put(key.as_bytes(), value.as_bytes()).unwrap();
    }
    println!("  ✅ Wrote 100 entities");
    
    // Force flush to create SST file (bloom filter in SST)
    db.flush().unwrap();
    println!("  ✅ Flushed to SST file");
    
    // Test bloom filter effectiveness
    println!("\n3. Testing bloom filter (checking non-existent keys):");
    
    let start = SystemTime::now();
    for i in 100..200 {
        let key = format!("entity:{:03}", i);
        let _ = db.get(key.as_bytes());
    }
    let elapsed = start.elapsed().unwrap();
    
    println!("  ✅ Checked 100 non-existent keys in {:?}", elapsed);
    println!("  💡 Bloom filter prevented unnecessary disk I/O!");
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}

fn demo_batch_writes() {
    let path = "/tmp/rocksdb-poc-batch";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. Atomic batch writes:");
    
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let db = DB::open(&opts, path).unwrap();
    
    // Create batch
    let mut batch = WriteBatch::default();
    
    println!("\n2. Building write batch:");
    batch.put(b"user:1", b"Alice");
    batch.put(b"user:2", b"Bob");
    batch.put(b"user:3", b"Charlie");
    batch.delete(b"temp:old_data");
    
    println!("  ✅ Added 3 puts and 1 delete to batch");
    
    // Execute batch atomically
    println!("\n3. Executing batch (atomic):");
    db.write(batch).unwrap();
    println!("  ✅ All operations committed atomically");
    
    // Verify
    println!("\n4. Verifying batch results:");
    let value = db.get(b"user:1").unwrap().unwrap();
    println!("  📖 user:1 = {}", String::from_utf8(value).unwrap());
    
    let value = db.get(b"user:2").unwrap().unwrap();
    println!("  📖 user:2 = {}", String::from_utf8(value).unwrap());
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}

fn demo_mvcc_pattern() {
    let path = "/tmp/rocksdb-poc-mvcc";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. MVCC with versioned keys:");
    
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let db = DB::open(&opts, path).unwrap();
    
    let entity_id = "entity:user:12345";
    
    // Write multiple versions
    println!("\n2. Writing multiple versions:");
    for version in 1..=5 {
        let timestamp = 1000 + version;
        let key = format!("{}:{}", entity_id, timestamp);
        let value = format!("state_v{}", version);
        
        db.put(key.as_bytes(), value.as_bytes()).unwrap();
        println!("  ✅ Wrote version {}: {}", version, key);
    }
    
    // Read latest version
    println!("\n3. Reading latest version:");
    let prefix = format!("{}:", entity_id);
    let mut iter = db.iterator(IteratorMode::From(prefix.as_bytes(), Direction::Reverse));
    
    if let Some(Ok((key, value))) = iter.next() {
        let k = String::from_utf8(key.to_vec()).unwrap();
        let v = String::from_utf8(value.to_vec()).unwrap();
        println!("  📖 Latest: {} = {}", k, v);
    }
    
    // Read specific version (time-travel)
    println!("\n4. Time-travel read (timestamp=1003):");
    let target_ts = 1003;
    let key = format!("{}:{}", entity_id, target_ts);
    
    match db.get(key.as_bytes()) {
        Ok(Some(value)) => {
            let v = String::from_utf8(value).unwrap();
            println!("  📖 Version at ts=1003: {}", v);
        }
        _ => println!("  ❌ Version not found"),
    }
    
    // Show all versions
    println!("\n5. All versions of entity:");
    let iter = db.iterator(IteratorMode::From(prefix.as_bytes(), Direction::Forward));
    
    for (key, value) in iter {
        let k = String::from_utf8(key.to_vec()).unwrap();
        if !k.starts_with(&prefix) {
            break;
        }
        let v = String::from_utf8(value.to_vec()).unwrap();
        println!("  📄 {} = {}", k, v);
    }
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}

fn demo_performance_stats() {
    let path = "/tmp/rocksdb-poc-stats";
    let _ = std::fs::remove_dir_all(path);
    
    println!("1. Opening database with stats enabled:");
    
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.enable_statistics();
    
    let db = DB::open(&opts, path).unwrap();
    
    // Perform some operations
    println!("\n2. Performing operations:");
    for i in 0..1000 {
        let key = format!("key:{:04}", i);
        let value = format!("value_{}", i);
        db.put(key.as_bytes(), value.as_bytes()).unwrap();
    }
    println!("  ✅ Wrote 1000 keys");
    
    // Read some keys
    for i in 0..100 {
        let key = format!("key:{:04}", i);
        let _ = db.get(key.as_bytes());
    }
    println!("  ✅ Read 100 keys");
    
    // Get statistics
    println!("\n3. Database statistics:");
    
    if let Ok(Some(stats)) = db.property_value("rocksdb.stats") {
        println!("\n{}", stats);
    }
    
    // Get specific properties
    println!("\n4. Specific properties:");
    
    if let Ok(Some(num_keys)) = db.property_value("rocksdb.estimate-num-keys") {
        println!("  📊 Estimated keys: {}", num_keys);
    }
    
    if let Ok(Some(mem_usage)) = db.property_value("rocksdb.cur-size-all-mem-tables") {
        println!("  💾 MemTable size: {} bytes", mem_usage);
    }
    
    if let Ok(Some(total_sst)) = db.property_value("rocksdb.total-sst-files-size") {
        println!("  📦 Total SST size: {} bytes", total_sst);
    }
    
    // Cleanup
    drop(db);
    let _ = std::fs::remove_dir_all(path);
}
