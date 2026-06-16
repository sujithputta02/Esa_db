/// Day 4 Example: Data Partitioning Strategies
/// 
/// This example demonstrates different ways to split data across servers.
/// We'll compare Range, Hash, and Consistent Hashing partitioning.
/// 
/// Real-world analogy: Different ways to organize books in a library!

use std::collections::HashMap;

// Sample user data
#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
}

// ============================================
// Strategy 1: Range Partitioning
// ============================================
struct RangePartitioner {
    ranges: Vec<(u64, u64, String)>, // (start, end, server_name)
}

impl RangePartitioner {
    fn new() -> Self {
        RangePartitioner {
            ranges: vec![
                (0, 249_999, "Server-1".to_string()),
                (250_000, 499_999, "Server-2".to_string()),
                (500_000, 749_999, "Server-3".to_string()),
                (750_000, 999_999, "Server-4".to_string()),
            ],
        }
    }

    fn get_server(&self, user_id: u64) -> String {
        for (start, end, server) in &self.ranges {
            if user_id >= *start && user_id <= *end {
                return server.clone();
            }
        }
        "Unknown".to_string()
    }
}

// ============================================
// Strategy 2: Hash Partitioning
// ============================================
struct HashPartitioner {
    num_servers: usize,
    servers: Vec<String>,
}

impl HashPartitioner {
    fn new(num_servers: usize) -> Self {
        let servers = (1..=num_servers)
            .map(|i| format!("Server-{}", i))
            .collect();
        
        HashPartitioner {
            num_servers,
            servers,
        }
    }

    fn get_server(&self, user_id: u64) -> String {
        // Simple hash: just use modulo
        let hash = user_id;
        let server_index = (hash % self.num_servers as u64) as usize;
        self.servers[server_index].clone()
    }
}

// ============================================
// Strategy 3: Consistent Hashing
// ============================================
struct ConsistentHashPartitioner {
    ring: Vec<(u64, String)>, // (position, server_name)
}

impl ConsistentHashPartitioner {
    fn new() -> Self {
        // Simplified ring with 4 servers at fixed positions
        let ring = vec![
            (0, "Server-1".to_string()),
            (250_000, "Server-2".to_string()),
            (500_000, "Server-3".to_string()),
            (750_000, "Server-4".to_string()),
        ];
        
        ConsistentHashPartitioner { ring }
    }

    fn get_server(&self, user_id: u64) -> String {
        let hash = user_id;
        
        // Find first server clockwise from hash position
        for (position, server) in &self.ring {
            if hash <= *position {
                return server.clone();
            }
        }
        
        // Wrap around to first server
        self.ring[0].1.clone()
    }

    fn add_server(&mut self, position: u64, server_name: String) {
        self.ring.push((position, server_name));
        self.ring.sort_by_key(|k| k.0);
    }
}

// ============================================
// Helper: Count distribution
// ============================================
fn count_distribution(placements: &[(u64, String)]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for (_, server) in placements {
        *counts.entry(server.clone()).or_insert(0) += 1;
    }
    counts
}

fn print_distribution(strategy_name: &str, counts: &HashMap<String, usize>, total: usize) {
    println!("📊 {} Distribution:", strategy_name);
    for i in 1..=4 {
        let server = format!("Server-{}", i);
        let count = counts.get(&server).unwrap_or(&0);
        let percentage = (*count as f64 / total as f64) * 100.0;
        let bar = "█".repeat((percentage / 5.0) as usize);
        println!("   {}: {:>5} users ({:>5.1}%) {}", 
                 server, count, percentage, bar);
    }
    println!();
}

fn main() {
    println!("🚀 Day 4 Example: Partitioning Strategies");
    println!("=========================================\n");

    println!("📚 Concept: Different ways to split data across servers!\n");

    // Create sample users
    let users: Vec<User> = vec![
        User { id: 12_345, name: "Alice".to_string() },
        User { id: 250_000, name: "Bob".to_string() },
        User { id: 500_000, name: "Carol".to_string() },
        User { id: 750_000, name: "Dave".to_string() },
        User { id: 123_456, name: "Eve".to_string() },
        User { id: 345_678, name: "Frank".to_string() },
        User { id: 567_890, name: "Grace".to_string() },
        User { id: 789_012, name: "Heidi".to_string() },
    ];

    println!("👥 Sample Users:");
    for user in &users {
        println!("   • {} (ID: {})", user.name, user.id);
    }
    println!();

    // ==========================================
    // PART 1: Range Partitioning
    // ==========================================
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📖 STRATEGY 1: Range Partitioning");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("🎯 Concept: Split by ID ranges (like a phonebook)");
    println!("   Server-1: 0-249,999");
    println!("   Server-2: 250,000-499,999");
    println!("   Server-3: 500,000-749,999");
    println!("   Server-4: 750,000-999,999\n");

    let range_partitioner = RangePartitioner::new();
    let mut range_placements = Vec::new();

    println!("📍 User Placement:");
    for user in &users {
        let server = range_partitioner.get_server(user.id);
        println!("   {} (ID: {}) → {}", user.name, user.id, server);
        range_placements.push((user.id, server));
    }
    println!();

    println!("✅ Pros:");
    println!("   • Simple to understand");
    println!("   • Range queries easy: 'Get all users 100,000-200,000'");
    println!("   • Good for time-series data");
    
    println!("\n❌ Cons:");
    println!("   • Can create hot spots!");
    println!("   • If all new users have high IDs → Server-4 overloaded");
    println!("   • Uneven distribution if some ranges more popular");

    // ==========================================
    // PART 2: Hash Partitioning
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔨 STRATEGY 2: Hash Partitioning");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("🎯 Concept: Hash the ID, use result % 4 to pick server");
    println!("   Formula: server = hash(user_id) % 4\n");

    let hash_partitioner = HashPartitioner::new(4);
    let mut hash_placements = Vec::new();

    println!("📍 User Placement:");
    for user in &users {
        let server = hash_partitioner.get_server(user.id);
        let hash = user.id % 4;
        println!("   {} (ID: {}) → hash % 4 = {} → {}", 
                 user.name, user.id, hash, server);
        hash_placements.push((user.id, server));
    }
    println!();

    println!("✅ Pros:");
    println!("   • Evenly distributed (no hot spots!)");
    println!("   • Simple algorithm");
    println!("   • Predictable (same key → same server)");
    
    println!("\n❌ Cons:");
    println!("   • Range queries impossible!");
    println!("   • 'Get all users 100,000-200,000' → must ask ALL servers");
    println!("   • Adding servers = rehash EVERYTHING (expensive!)");

    // ==========================================
    // PART 3: Consistent Hashing
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 STRATEGY 3: Consistent Hashing");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("🎯 Concept: Ring-based with minimal data movement");
    println!("   Ring positions:");
    println!("      0 → Server-1");
    println!("    250,000 → Server-2");
    println!("    500,000 → Server-3");
    println!("    750,000 → Server-4\n");

    let mut consistent_partitioner = ConsistentHashPartitioner::new();
    let mut consistent_placements = Vec::new();

    println!("📍 User Placement (Before adding new server):");
    for user in &users {
        let server = consistent_partitioner.get_server(user.id);
        println!("   {} (ID: {}) → {}", user.name, user.id, server);
        consistent_placements.push((user.id, server.clone()));
    }
    println!();

    // Simulate adding a new server
    println!("➕ ADDING NEW SERVER at position 625,000...\n");
    consistent_partitioner.add_server(625_000, "Server-5".to_string());

    println!("📍 User Placement (After adding Server-5):");
    let mut moved = 0;
    for user in &users {
        let new_server = consistent_partitioner.get_server(user.id);
        let old_server = consistent_placements
            .iter()
            .find(|(id, _)| *id == user.id)
            .map(|(_, s)| s.as_str())
            .unwrap_or("Unknown");
        
        if old_server != new_server {
            println!("   {} (ID: {}) → {} (MOVED from {})", 
                     user.name, user.id, new_server, old_server);
            moved += 1;
        } else {
            println!("   {} (ID: {}) → {} (stayed)", 
                     user.name, user.id, new_server);
        }
    }
    println!();

    println!("✅ Pros:");
    println!("   • Add server → only ~25% of data moves");
    println!("   • Remove server → only its data moves");
    println!("   • Scales elastically");
    println!("   • Perfect for ESA!");
    
    println!("\n📊 Movement Stats:");
    println!("   • {} out of {} users moved ({:.1}%)", 
             moved, users.len(), (moved as f64 / users.len() as f64) * 100.0);
    println!("   • Hash partitioning would move 100% of data!");

    // ==========================================
    // PART 4: Large-Scale Distribution Test
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 LARGE-SCALE DISTRIBUTION TEST");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("Testing with 10,000 users to see distribution quality...\n");

    let test_users: Vec<u64> = (0..10_000).map(|i| i * 100).collect();

    // Test Range Partitioning
    let range_test: Vec<(u64, String)> = test_users
        .iter()
        .map(|&id| (id, range_partitioner.get_server(id)))
        .collect();
    let range_dist = count_distribution(&range_test);
    print_distribution("Range Partitioning", &range_dist, test_users.len());

    // Test Hash Partitioning
    let hash_test: Vec<(u64, String)> = test_users
        .iter()
        .map(|&id| (id, hash_partitioner.get_server(id)))
        .collect();
    let hash_dist = count_distribution(&hash_test);
    print_distribution("Hash Partitioning", &hash_dist, test_users.len());

    // Show summary
    println!("\n┌────────────────────────────────────┐");
    println!("│  💡 KEY TAKEAWAYS                 │");
    println!("└────────────────────────────────────┘\n");

    println!("1️⃣  Range Partitioning:");
    println!("   • Best for: Time-series, sequential IDs");
    println!("   • Watch out for: Hot spots on popular ranges\n");

    println!("2️⃣  Hash Partitioning:");
    println!("   • Best for: Even load distribution");
    println!("   • Watch out for: Can't do range queries\n");

    println!("3️⃣  Consistent Hashing:");
    println!("   • Best for: Dynamic clusters (ESA uses this!)");
    println!("   • Watch out for: Slightly more complex\n");

    println!("🎯 ESA's Choice: Consistent Hashing!");
    println!("   Why? AI can add/remove servers dynamically");
    println!("   with minimal data movement!\n");
}
