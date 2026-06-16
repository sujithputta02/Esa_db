// Consistent Hashing Example - The Smart Locker System
// This shows how to distribute data smartly across servers!

use std::collections::{HashMap, BTreeMap};

/// A simple hash function (like a locker number calculator)
fn simple_hash(key: &str) -> u32 {
    // Simple hash: sum of character codes
    key.chars().map(|c| c as u32).sum::<u32>() % 360  // 360 positions like degrees in a circle
}

/// A consistent hash ring (like a circular hallway of lockers)
struct HashRing {
    // Maps position on circle → server name
    ring: BTreeMap<u32, String>,
    // Track which data is on which server
    data_locations: HashMap<String, String>,
}

impl HashRing {
    fn new() -> Self {
        println!("🎪 Creating hash ring (like a circular hallway)");
        HashRing {
            ring: BTreeMap::new(),
            data_locations: HashMap::new(),
        }
    }

    /// Add a server to the ring
    fn add_server(&mut self, server_name: String) {
        // Hash server name to get position on circle (0-360 degrees)
        let position = simple_hash(&server_name);

        println!("➕ Adding {} at position {} degrees", server_name, position);
        self.ring.insert(position, server_name.clone());

        // Redistribute data affected by this new server
        self.redistribute_data(&server_name);
    }

    /// Remove a server from the ring
    fn remove_server(&mut self, server_name: &str) {
        // Find and remove the server
        let position_to_remove = self.ring.iter()
            .find(|(_, name)| *name == server_name)
            .map(|(pos, _)| *pos);

        if let Some(pos) = position_to_remove {
            self.ring.remove(&pos);
            println!("➖ Removed {} from position {}", server_name, pos);

            // Move data to next server
            self.redistribute_after_removal(server_name);
        }
    }

    /// Find which server should store this data
    fn get_server(&self, data_key: &str) -> Option<String> {
        if self.ring.is_empty() {
            return None;
        }

        // Hash the data key to get position on circle
        let position = simple_hash(data_key);

        // Find first server clockwise from this position
        for (&server_pos, server_name) in self.ring.iter() {
            if server_pos >= position {
                return Some(server_name.clone());
            }
        }

        // Wrap around to first server
        self.ring.values().next().cloned()
    }

    /// Store data on the ring
    fn store_data(&mut self, data_key: String) {
        if let Some(server) = self.get_server(&data_key) {
            let position = simple_hash(&data_key);
            println!("📦 Storing '{}' (pos {}) → {}", data_key, position, server);
            self.data_locations.insert(data_key, server);
        }
    }

    /// Redistribute data when adding a server
    fn redistribute_data(&mut self, new_server: &str) {
        let new_pos = simple_hash(new_server);
        let mut moved_count = 0;

        // Check all data to see if it should move to new server
        let keys: Vec<String> = self.data_locations.keys().cloned().collect();

        for key in keys {
            let data_pos = simple_hash(&key);
            let current_server = self.data_locations.get(&key).unwrap();

            // Should this data move to the new server?
            if data_pos <= new_pos && current_server != new_server {
                // Check if new server is closer (clockwise)
                if let Some(new_owner) = self.get_server(&key) {
                    if &new_owner != current_server {
                        println!("  🔄 Moving '{}' from {} → {}",
                                 key, current_server, new_owner);
                        self.data_locations.insert(key, new_owner);
                        moved_count += 1;
                    }
                }
            }
        }

        if moved_count > 0 {
            println!("  ✅ Moved {} items to new server", moved_count);
        } else {
            println!("  ℹ️  No data needed to move");
        }
    }

    /// Redistribute data when removing a server
    fn redistribute_after_removal(&mut self, removed_server: &str) {
        let keys: Vec<String> = self.data_locations.keys().cloned().collect();
        let mut moved_count = 0;

        for key in keys {
            let current_server = self.data_locations.get(&key).unwrap();

            if current_server == removed_server {
                // Data was on removed server, move to next server
                if let Some(new_server) = self.get_server(&key) {
                    println!("  🔄 Moving '{}' from {} → {}",
                             key, removed_server, new_server);
                    self.data_locations.insert(key, new_server);
                    moved_count += 1;
                }
            }
        }

        println!("  ✅ Moved {} items from removed server", moved_count);
    }

    /// Show current distribution
    fn show_distribution(&self) {
        println!("\n📊 Current Distribution:");
        let mut server_counts: HashMap<String, usize> = HashMap::new();

        for server in self.data_locations.values() {
            *server_counts.entry(server.clone()).or_insert(0) += 1;
        }

        for (server, count) in server_counts.iter() {
            println!("  {}: {} items", server, count);
        }
        println!("  Total: {} items", self.data_locations.len());
    }

    /// Show the ring structure
    fn show_ring(&self) {
        println!("\n🎪 Ring Structure (clockwise):");
        for (position, server) in self.ring.iter() {
            println!("  {}° → {}", position, server);
        }
    }
}

fn main() {
    println!("=== Consistent Hashing Demo ===");
    println!("(Like smart lockers in a circular hallway!)\n");

    let mut ring = HashRing::new();

    // Scenario 1: Add 3 servers
    println!("\n--- Scenario 1: Building Initial Cluster ---");
    ring.add_server("ServerA".to_string());
    ring.add_server("ServerB".to_string());
    ring.add_server("ServerC".to_string());
    ring.show_ring();

    // Scenario 2: Store some data
    println!("\n--- Scenario 2: Storing User Data ---");
    let users = vec![
        "user_001", "user_002", "user_003", "user_004", "user_005",
        "user_006", "user_007", "user_008", "user_009", "user_010",
    ];

    for user in &users {
        ring.store_data(user.to_string());
    }
    ring.show_distribution();

    // Scenario 3: Add a new server (scale up!)
    println!("\n--- Scenario 3: Adding New Server (Scaling Up) ---");
    println!("Business is growing! Adding ServerD...");
    ring.add_server("ServerD".to_string());
    ring.show_ring();
    ring.show_distribution();

    println!("\n💡 Notice: Only SOME data moved, not everything!");
    println!("   With regular hashing, ALL data would move!");

    // Scenario 4: Remove a server (server failure)
    println!("\n--- Scenario 4: Removing Server (Failure) ---");
    println!("ServerB failed! Removing it...");
    ring.remove_server("ServerB");
    ring.show_ring();
    ring.show_distribution();

    println!("\n💡 Notice: Only ServerB's data moved, others stayed!");

    // Scenario 5: Look up where specific data is
    println!("\n--- Scenario 5: Data Lookup ---");
    for user in &users[0..3] {
        if let Some(server) = ring.get_server(user) {
            println!("  '{}' is on {}", user, server);
        }
    }

    // Final summary
    println!("\n\n🎯 Key Points:");
    println!("✓ Data distributed evenly across servers");
    println!("✓ Add server → only nearby data moves (not all!)");
    println!("✓ Remove server → only its data moves (not all!)");
    println!("✓ Fast lookup: O(log N) to find which server has data");
    println!("\nWhy ESA needs this:");
    println!("→ Scale cluster easily (add/remove servers anytime)");
    println!("→ Minimal data movement (saves time and bandwidth)");
    println!("→ AI knows exactly where data lives (fast decisions)");
    println!("→ Automatic rebalancing (no manual work needed)");

    // Calculate efficiency
    let total_items = users.len();
    let expected_moved = total_items / 4;  // With 4 servers, ~25% per server
    println!("\n📈 Efficiency:");
    println!("  Traditional hash: {} items would move (100%)", total_items);
    println!("  Consistent hash: ~{} items moved (~25%)", expected_moved);
    println!("  Savings: ~75% less data movement! 🎉");
}
