/// Day 4 Example: Rebalancing Data When Servers Change
/// 
/// This example shows what happens when you add or remove servers.
/// We'll see how much data needs to move in different scenarios.
/// 
/// Real-world analogy: Reorganizing books when shelves are added/removed!

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DataItem {
    id: u64,
    _name: String, // Prefixed with _ to indicate intentionally unused in this demo
    size_mb: u64,
}

#[derive(Debug)]
struct Server {
    name: String,
    data: Vec<DataItem>,
    total_size_mb: u64,
}

impl Server {
    fn new(name: &str) -> Self {
        Server {
            name: name.to_string(),
            data: Vec::new(),
            total_size_mb: 0,
        }
    }

    fn add_data(&mut self, item: DataItem) {
        self.total_size_mb += item.size_mb;
        self.data.push(item);
    }

    fn remove_data(&mut self, id: u64) -> Option<DataItem> {
        if let Some(pos) = self.data.iter().position(|item| item.id == id) {
            let item = self.data.remove(pos);
            self.total_size_mb -= item.size_mb;
            Some(item)
        } else {
            None
        }
    }

    fn print_status(&self) {
        let bar = "█".repeat((self.total_size_mb / 10) as usize);
        println!("   {}: {} MB [{}]", self.name, self.total_size_mb, bar);
        println!("      {} items", self.data.len());
    }
}

struct Cluster {
    servers: HashMap<String, Server>,
}

impl Cluster {
    fn new() -> Self {
        Cluster {
            servers: HashMap::new(),
        }
    }

    fn add_server(&mut self, name: &str) {
        self.servers.insert(name.to_string(), Server::new(name));
    }

    fn place_data(&mut self, item: DataItem) {
        // Simple hash-based placement
        let server_count = self.servers.len();
        let server_index = (item.id as usize) % server_count;
        let server_name = format!("Server-{}", server_index + 1);
        
        if let Some(server) = self.servers.get_mut(&server_name) {
            server.add_data(item);
        }
    }

    fn print_status(&self, title: &str) {
        println!("\n📊 {}", title);
        println!("   Servers: {}", self.servers.len());
        for i in 1..=self.servers.len() {
            let server_name = format!("Server-{}", i);
            if let Some(server) = self.servers.get(&server_name) {
                server.print_status();
            }
        }
        let total: u64 = self.servers.values().map(|s| s.total_size_mb).sum();
        let avg = total / self.servers.len() as u64;
        println!("   Total: {} MB | Average: {} MB per server", total, avg);
    }

    fn rebalance_after_adding_server(&mut self, new_server_name: &str) -> (usize, u64) {
        println!("\n🔄 Starting rebalancing...");
        
        let mut items_to_move = Vec::new();
        let server_count = self.servers.len();

        // Find items that should move to different server with new count
        for (server_name, server) in self.servers.iter() {
            if server_name != new_server_name {
                for item in &server.data {
                    let new_placement = ((item.id as usize) % server_count) + 1;
                    let new_server = format!("Server-{}", new_placement);
                    
                    if new_server != *server_name {
                        items_to_move.push((item.clone(), server_name.clone(), new_server));
                    }
                }
            }
        }

        let moved_count = items_to_move.len();
        let mut moved_size = 0u64;

        // Perform the moves
        for (item, from_server, to_server) in items_to_move {
            moved_size += item.size_mb;
            
            // Remove from old server
            if let Some(server) = self.servers.get_mut(&from_server) {
                server.remove_data(item.id);
            }
            
            // Add to new server
            if let Some(server) = self.servers.get_mut(&to_server) {
                server.add_data(item);
            }
        }

        println!("   ✅ Moved {} items ({} MB)", moved_count, moved_size);
        (moved_count, moved_size)
    }
}

fn main() {
    println!("🚀 Day 4 Example: Rebalancing When Servers Change");
    println!("=================================================\n");

    println!("📚 Concept: What happens when you add/remove servers?\n");

    // Create sample data
    let mut all_data = Vec::new();
    for i in 0..100 {
        all_data.push(DataItem {
            id: i,
            _name: format!("data_{}", i),
            size_mb: 10, // Each item is 10 MB
        });
    }

    println!("📦 Sample Data: 100 items × 10 MB = 1,000 MB total\n");

    // ==========================================
    // SCENARIO 1: Start with 3 servers
    // ==========================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📍 SCENARIO 1: Initial Setup (3 Servers)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut cluster = Cluster::new();
    cluster.add_server("Server-1");
    cluster.add_server("Server-2");
    cluster.add_server("Server-3");

    // Distribute data across 3 servers
    for item in all_data.clone() {
        cluster.place_data(item);
    }

    cluster.print_status("Initial Distribution (3 servers)");

    println!("\n💡 Expected: Each server has ~33% of data (333 MB)");

    // ==========================================
    // SCENARIO 2: Add a 4th server
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("➕ SCENARIO 2: Adding 4th Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n🎯 Goal: Balance to 25% per server (250 MB each)");
    println!("   Need to move: ~25% of total data\n");

    cluster.add_server("Server-4");
    let (moved_items, moved_mb) = cluster.rebalance_after_adding_server("Server-4");

    cluster.print_status("After Adding Server-4");

    let total_items = 100;
    let total_mb = 1000;
    let moved_percent = (moved_items as f64 / total_items as f64) * 100.0;
    
    println!("\n📊 Movement Statistics:");
    println!("   • Items moved: {} / {} ({:.1}%)", 
             moved_items, total_items, moved_percent);
    println!("   • Data moved: {} / {} MB ({:.1}%)", 
             moved_mb, total_mb, (moved_mb as f64 / total_mb as f64) * 100.0);

    // ==========================================
    // SCENARIO 3: Server Failure
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💀 SCENARIO 3: Server-3 Fails!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n🚨 Server-3 crashed! Need to redistribute its data...");

    // Get Server-3's data before removing it
    let failed_data = if let Some(server) = cluster.servers.get("Server-3") {
        server.data.clone()
    } else {
        Vec::new()
    };

    let failed_size = failed_data.iter().map(|d| d.size_mb).sum::<u64>();
    let failed_count = failed_data.len();

    println!("   Lost: {} items ({} MB)", failed_count, failed_size);

    // Remove failed server
    cluster.servers.remove("Server-3");

    println!("\n🔄 Redistributing to remaining 3 servers...\n");

    // Redistribute failed server's data
    for item in failed_data {
        cluster.place_data(item);
    }

    cluster.print_status("After Server-3 Failure");

    println!("\n💡 With Replication (Smart Way):");
    println!("   ✅ If Server-3 had replicas on other servers");
    println!("   ✅ Just promote replicas to primary");
    println!("   ✅ Rebalancing complete in SECONDS!");
    println!("   ✅ No data movement needed!");

    // ==========================================
    // COMPARISON: Different Strategies
    // ==========================================
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 COMPARISON: Rebalancing Strategies");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Adding 1 server to a 3-server cluster (1000 MB total):\n");

    println!("1️⃣  Naive Rehashing (Simple Hash):");
    println!("   • Data moved: ~750 MB (75%)");
    println!("   • Items moved: ~75 items");
    println!("   • Time: Long (depends on network)");
    println!("   • Impact: HIGH - users see slowness");
    println!("   • Reason: Hash changes for most keys\n");

    println!("2️⃣  Minimal Rebalancing:");
    println!("   • Data moved: ~250 MB (25%)");
    println!("   • Items moved: ~25 items");
    println!("   • Time: Medium");
    println!("   • Impact: MEDIUM - some latency");
    println!("   • Reason: Only move what's needed for balance\n");

    println!("3️⃣  Consistent Hashing:");
    println!("   • Data moved: ~125 MB (12.5%)");
    println!("   • Items moved: ~12-13 items");
    println!("   • Time: Short");
    println!("   • Impact: LOW - barely noticeable");
    println!("   • Reason: Only nearby keys on ring move\n");

    println!("4️⃣  With Replication (Best!):");
    println!("   • Data moved: 0 MB (0%)");
    println!("   • Items moved: 0 items");
    println!("   • Time: Instant (1-2 seconds)");
    println!("   • Impact: NONE - seamless");
    println!("   • Reason: Replicas already exist!\n");

    // ==========================================
    // ESA's Approach
    // ==========================================
    println!("\n┌────────────────────────────────────────────────┐");
    println!("│  🎯 HOW ESA HANDLES REBALANCING               │");
    println!("└────────────────────────────────────────────────┘\n");

    println!("1. AI Detects Load Issue:");
    println!("   📊 \"Server-2 is at 90% capacity!\"");
    println!("   🤖 \"I should add a new server or rebalance\"\n");

    println!("2. Safety Layer Checks:");
    println!("   🛡️ \"Is this operation safe?\"");
    println!("   📋 \"Check policy: max_data_movement = 30%\"");
    println!("   ✅ \"Consistent hashing moves only 12.5% - APPROVED\"\n");

    println!("3. Execute Rebalancing:");
    println!("   🔨 \"Adding Server-5 at ring position 225,000\"");
    println!("   📦 \"Moving 12-13 items (125 MB)\"");
    println!("   ⏱️ \"Estimated time: 30 seconds\"\n");

    println!("4. Monitor Progress:");
    println!("   📈 \"Migration 50% complete...\"");
    println!("   📈 \"Migration 100% complete!\"");
    println!("   ✅ \"Cluster rebalanced successfully\"\n");

    println!("5. Audit & Report:");
    println!("   📝 \"Logged: AI added server due to high load\"");
    println!("   📊 \"Result: Load reduced from 90% to 70%\"");
    println!("   🎉 \"Users experienced no downtime!\"\n");

    // ==========================================
    // Key Takeaways
    // ==========================================
    println!("┌────────────────────────────────────────────────┐");
    println!("│  💡 KEY TAKEAWAYS                              │");
    println!("└────────────────────────────────────────────────┘\n");

    println!("✅ GOOD Practices:");
    println!("   • Use consistent hashing (minimal movement)");
    println!("   • Have replicas ready (instant failover)");
    println!("   • Rebalance gradually (don't impact users)");
    println!("   • Monitor during migration (detect issues early)\n");

    println!("❌ BAD Practices:");
    println!("   • Simple hash % N (moves too much data)");
    println!("   • No replication (lose data on failure)");
    println!("   • Move everything at once (overwhelms network)");
    println!("   • No monitoring (can't detect problems)\n");

    println!("🎯 ESA's Strategy:");
    println!("   • Consistent Hashing + Replication");
    println!("   • AI-driven but policy-bounded");
    println!("   • Gradual migration with monitoring");
    println!("   • Full audit trail of all changes");
    println!("   • Users never see downtime! 🎉\n");
}
