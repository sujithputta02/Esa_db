/// Day 5: Storage Basics Example
/// 
/// This example demonstrates:
/// 1. LSM tree concepts (MemTable → SSTable)
/// 2. RocksDB column families
/// 3. MVCC with versioned keys
/// 4. WAL and crash recovery

use std::time::{SystemTime, UNIX_EPOCH};

/// Simple in-memory LSM tree simulation
mod lsm_simulation {
    use std::collections::BTreeMap;
    
    /// MemTable: In-memory sorted buffer
    pub struct MemTable {
        data: BTreeMap<String, String>,
        size: usize,
        max_size: usize,
    }
    
    impl MemTable {
        pub fn new(max_size: usize) -> Self {
            Self {
                data: BTreeMap::new(),
                size: 0,
                max_size,
            }
        }
        
        /// Write to MemTable (fast, in-memory)
        pub fn put(&mut self, key: String, value: String) -> bool {
            let entry_size = key.len() + value.len();
            
            if self.size + entry_size > self.max_size {
                return false; // MemTable full, needs flush
            }
            
            self.data.insert(key, value);
            self.size += entry_size;
            true
        }
        
        /// Read from MemTable
        pub fn get(&self, key: &str) -> Option<&String> {
            self.data.get(key)
        }
        
        /// Check if MemTable is full
        #[allow(dead_code)]
        pub fn is_full(&self) -> bool {
            self.size >= self.max_size
        }
        
        /// Get all entries (for flushing to SSTable)
        pub fn entries(&self) -> Vec<(String, String)> {
            self.data.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
        
        pub fn len(&self) -> usize {
            self.data.len()
        }
    }
    
    /// SSTable: Immutable on-disk sorted file (simulated)
    pub struct SSTable {
        id: usize,
        data: BTreeMap<String, String>,
        min_key: String,
        max_key: String,
    }
    
    impl SSTable {
        /// Create SSTable from MemTable flush
        pub fn from_memtable(id: usize, entries: Vec<(String, String)>) -> Self {
            let mut data = BTreeMap::new();
            for (k, v) in entries {
                data.insert(k, v);
            }
            
            let min_key = data.keys().next().unwrap().clone();
            let max_key = data.keys().last().unwrap().clone();
            
            Self { id, data, min_key, max_key }
        }
        
        /// Read from SSTable
        pub fn get(&self, key: &str) -> Option<&String> {
            // In real LSM tree, would use bloom filter here
            self.data.get(key)
        }
        
        /// Check if key might be in range (bloom filter simulation)
        pub fn might_contain(&self, key: &str) -> bool {
            key >= self.min_key.as_str() && key <= self.max_key.as_str()
        }
    }
    
    /// Simple LSM storage engine
    pub struct LSMStorage {
        memtable: MemTable,
        sstables: Vec<SSTable>,
        next_sstable_id: usize,
    }
    
    impl LSMStorage {
        pub fn new(memtable_size: usize) -> Self {
            Self {
                memtable: MemTable::new(memtable_size),
                sstables: Vec::new(),
                next_sstable_id: 0,
            }
        }
        
        /// Write key-value (goes to MemTable first)
        pub fn put(&mut self, key: String, value: String) {
            if !self.memtable.put(key.clone(), value.clone()) {
                // MemTable full, flush to SSTable
                println!("  [LSM] MemTable full, flushing to SSTable-{}...", self.next_sstable_id);
                self.flush_memtable();
                
                // Try again with new MemTable
                self.memtable.put(key, value);
            }
        }
        
        /// Flush MemTable to SSTable
        fn flush_memtable(&mut self) {
            let entries = self.memtable.entries();
            let sstable = SSTable::from_memtable(self.next_sstable_id, entries);
            
            println!("  [LSM] Created SSTable-{} with {} entries", 
                     sstable.id, sstable.data.len());
            
            self.sstables.push(sstable);
            self.next_sstable_id += 1;
            
            // Create new MemTable
            self.memtable = MemTable::new(self.memtable.size);
        }
        
        /// Read key-value (check MemTable → SSTables)
        pub fn get(&self, key: &str) -> Option<String> {
            // 1. Check MemTable first (newest data)
            if let Some(value) = self.memtable.get(key) {
                println!("  [LSM] Found '{}' in MemTable", key);
                return Some(value.clone());
            }
            
            // 2. Check SSTables (newest to oldest)
            for sstable in self.sstables.iter().rev() {
                // Bloom filter check (simulated)
                if !sstable.might_contain(key) {
                    println!("  [LSM] Bloom filter: '{}' not in SSTable-{}", key, sstable.id);
                    continue;
                }
                
                if let Some(value) = sstable.get(key) {
                    println!("  [LSM] Found '{}' in SSTable-{}", key, sstable.id);
                    return Some(value.clone());
                }
            }
            
            println!("  [LSM] Key '{}' not found", key);
            None
        }
        
        pub fn stats(&self) {
            println!("  [LSM] MemTable: {} entries, {} bytes", 
                     self.memtable.len(), self.memtable.size);
            println!("  [LSM] SSTables: {} files", self.sstables.len());
        }
    }
}

/// MVCC (Multi-Version Concurrency Control) simulation
mod mvcc_simulation {
    use std::collections::BTreeMap;
    
    pub type Timestamp = u64;
    
    /// Versioned value
    #[derive(Clone, Debug)]
    pub struct Version {
        pub timestamp: Timestamp,
        pub value: String,
    }
    
    /// MVCC store
    pub struct MVCCStore {
        // key → sorted list of versions
        data: BTreeMap<String, Vec<Version>>,
        current_ts: Timestamp,
    }
    
    impl MVCCStore {
        pub fn new() -> Self {
            Self {
                data: BTreeMap::new(),
                current_ts: 1,
            }
        }
        
        /// Write new version (creates new timestamp)
        pub fn write(&mut self, key: String, value: String) -> Timestamp {
            let ts = self.current_ts;
            self.current_ts += 1;
            
            let version = Version { timestamp: ts, value };
            
            self.data.entry(key.clone())
                .or_insert_with(Vec::new)
                .push(version);
            
            println!("  [MVCC] Wrote '{}' at timestamp {}", key, ts);
            ts
        }
        
        /// Read at specific timestamp (snapshot read)
        pub fn read_at(&self, key: &str, ts: Timestamp) -> Option<String> {
            if let Some(versions) = self.data.get(key) {
                // Find latest version <= ts
                for version in versions.iter().rev() {
                    if version.timestamp <= ts {
                        println!("  [MVCC] Read '{}' at ts={} → found version at ts={}", 
                                 key, ts, version.timestamp);
                        return Some(version.value.clone());
                    }
                }
            }
            
            println!("  [MVCC] No version of '{}' found at ts={}", key, ts);
            None
        }
        
        /// Read latest version
        pub fn read_latest(&self, key: &str) -> Option<String> {
            self.read_at(key, self.current_ts)
        }
        
        /// Show all versions of a key
        pub fn show_versions(&self, key: &str) {
            if let Some(versions) = self.data.get(key) {
                println!("  [MVCC] All versions of '{}':", key);
                for v in versions {
                    println!("    ts={}: {}", v.timestamp, v.value);
                }
            }
        }
        
        /// Garbage collect old versions (keep last N)
        pub fn gc(&mut self, key: &str, keep_last: usize) {
            if let Some(versions) = self.data.get_mut(key) {
                if versions.len() > keep_last {
                    let remove_count = versions.len() - keep_last;
                    versions.drain(0..remove_count);
                    println!("  [MVCC] GC: Removed {} old versions of '{}'", remove_count, key);
                }
            }
        }
    }
}

/// Column Family simulation
mod column_family_simulation {
    use std::collections::HashMap;
    
    pub struct ColumnFamily {
        #[allow(dead_code)]
        name: String,
        data: HashMap<String, String>,
    }
    
    impl ColumnFamily {
        pub fn new(name: String) -> Self {
            Self {
                name,
                data: HashMap::new(),
            }
        }
        
        pub fn put(&mut self, key: String, value: String) {
            self.data.insert(key, value);
        }
        
        pub fn get(&self, key: &str) -> Option<&String> {
            self.data.get(key)
        }
        
        #[allow(dead_code)]
        pub fn name(&self) -> &str {
            &self.name
        }
    }
    
    /// Multi-column family database
    pub struct CFDatabase {
        column_families: HashMap<String, ColumnFamily>,
    }
    
    impl CFDatabase {
        pub fn new() -> Self {
            Self {
                column_families: HashMap::new(),
            }
        }
        
        pub fn create_cf(&mut self, name: String) {
            let cf = ColumnFamily::new(name.clone());
            self.column_families.insert(name.clone(), cf);
            println!("  [CF] Created column family: {}", name);
        }
        
        pub fn put_cf(&mut self, cf_name: &str, key: String, value: String) {
            if let Some(cf) = self.column_families.get_mut(cf_name) {
                cf.put(key.clone(), value);
                println!("  [CF] Put '{}' in CF '{}'", key, cf_name);
            }
        }
        
        pub fn get_cf(&self, cf_name: &str, key: &str) -> Option<String> {
            self.column_families.get(cf_name)
                .and_then(|cf| cf.get(key))
                .cloned()
        }
        
        pub fn list_cfs(&self) {
            println!("  [CF] Column families:");
            for (name, cf) in &self.column_families {
                println!("    - {}: {} keys", name, cf.data.len());
            }
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Day 5: Storage Systems Concepts Demo              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // Demo 1: LSM Tree Basics
    println!("━━━ Demo 1: LSM Tree (MemTable → SSTable) ━━━");
    demo_lsm_tree();
    
    // Demo 2: MVCC
    println!("\n━━━ Demo 2: MVCC (Multi-Version Concurrency Control) ━━━");
    demo_mvcc();
    
    // Demo 3: Column Families
    println!("\n━━━ Demo 3: Column Families ━━━");
    demo_column_families();
    
    // Demo 4: ESA Use Case
    println!("\n━━━ Demo 4: ESA Entity Storage Pattern ━━━");
    demo_esa_pattern();
}

fn demo_lsm_tree() {
    use lsm_simulation::LSMStorage;
    
    // Create LSM storage with small MemTable (100 bytes)
    let mut storage = LSMStorage::new(100);
    
    println!("\n1. Writing data (goes to MemTable):");
    storage.put("entity:1".to_string(), "state_v1".to_string());
    storage.put("entity:2".to_string(), "state_v2".to_string());
    storage.put("entity:3".to_string(), "state_v3".to_string());
    
    println!("\n2. Storage stats:");
    storage.stats();
    
    println!("\n3. Continue writing (triggers flush):");
    storage.put("entity:4".to_string(), "state_v4".to_string());
    storage.put("entity:5".to_string(), "state_v5".to_string());
    
    println!("\n4. Reading data:");
    storage.get("entity:1"); // From SSTable
    storage.get("entity:5"); // From MemTable
    storage.get("entity:99"); // Not found
    
    println!("\n5. Final stats:");
    storage.stats();
}

fn demo_mvcc() {
    use mvcc_simulation::MVCCStore;
    
    let mut store = MVCCStore::new();
    
    println!("\n1. Write multiple versions of same entity:");
    let ts1 = store.write("entity:123".to_string(), "v1: initial state".to_string());
    let ts2 = store.write("entity:123".to_string(), "v2: updated state".to_string());
    let ts3 = store.write("entity:123".to_string(), "v3: final state".to_string());
    
    println!("\n2. Show all versions:");
    store.show_versions("entity:123");
    
    println!("\n3. Time-travel reads (snapshot isolation):");
    store.read_at("entity:123", ts1);
    store.read_at("entity:123", ts2);
    store.read_at("entity:123", ts3);
    
    println!("\n4. Read latest:");
    store.read_latest("entity:123");
    
    println!("\n5. Garbage collection (keep last 2 versions):");
    store.gc("entity:123", 2);
    store.show_versions("entity:123");
}

fn demo_column_families() {
    use column_family_simulation::CFDatabase;
    
    let mut db = CFDatabase::new();
    
    println!("\n1. Create column families (like ESA design):");
    db.create_cf("entities".to_string());
    db.create_cf("metadata".to_string());
    db.create_cf("raft_log".to_string());
    
    println!("\n2. Write to different CFs:");
    db.put_cf("entities", "entity:123".to_string(), "state_data".to_string());
    db.put_cf("metadata", "entity:123".to_string(), "security_policy".to_string());
    db.put_cf("raft_log", "entry:1".to_string(), "log_entry_1".to_string());
    
    println!("\n3. Read from different CFs:");
    if let Some(value) = db.get_cf("entities", "entity:123") {
        println!("  [CF] Read from entities: {}", value);
    }
    if let Some(value) = db.get_cf("metadata", "entity:123") {
        println!("  [CF] Read from metadata: {}", value);
    }
    
    println!("\n4. List all CFs:");
    db.list_cfs();
}

fn demo_esa_pattern() {
    use column_family_simulation::CFDatabase;
    
    println!("\n1. ESA Entity Storage Pattern:");
    println!("   - Use column families for logical separation");
    println!("   - Use MVCC for versioning");
    println!("   - Use LSM tree for write-heavy workloads");
    
    println!("\n2. Simulated entity write:");
    let entity_id = "entity:user:12345";
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Versioned key format
    let versioned_key = format!("{}:{}", entity_id, timestamp);
    
    println!("  Entity ID: {}", entity_id);
    println!("  Timestamp: {}", timestamp);
    println!("  Versioned Key: {}", versioned_key);
    
    println!("\n3. Write to entities CF:");
    let mut db = CFDatabase::new();
    db.create_cf("entities".to_string());
    db.put_cf("entities", versioned_key.clone(), "entity_state_data".to_string());
    
    println!("\n4. Read latest version:");
    if let Some(state) = db.get_cf("entities", &versioned_key) {
        println!("  Retrieved state: {}", state);
    }
    
    println!("\n5. ESA Storage Benefits:");
    println!("  ✓ High write throughput (LSM tree)");
    println!("  ✓ Point-in-time reads (MVCC)");
    println!("  ✓ Logical separation (Column Families)");
    println!("  ✓ Crash recovery (WAL)");
    println!("  ✓ Efficient compaction (Background)");
}
