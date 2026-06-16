// CRDT G-Counter Example - The Magic Like Counter
// This shows how counters can automatically merge without conflicts!

/// A G-Counter (Grow-only Counter) - like YouTube likes
/// Each server has its own count, merge by taking the maximum
#[derive(Debug, Clone)]
struct GCounter {
    server_id: usize,
    // Each server tracks its own count
    // Index = server_id, Value = count from that server
    counts: Vec<u32>,
}

impl GCounter {
    /// Create a new counter for a specific server
    fn new(server_id: usize, num_servers: usize) -> Self {
        println!("📊 Created counter for Server {}", server_id);
        GCounter {
            server_id,
            counts: vec![0; num_servers],
        }
    }

    /// This server adds some likes
    fn increment(&mut self, amount: u32) {
        self.counts[self.server_id] += amount;
        println!("  Server {} added {} likes (local count: {})",
                 self.server_id, amount, self.counts[self.server_id]);
    }

    /// Get total likes (sum all servers)
    fn value(&self) -> u32 {
        self.counts.iter().sum()
    }

    /// Merge with another counter (the magic part!)
    fn merge(&mut self, other: &GCounter) {
        println!("\n🔄 Server {} merging with Server {}...",
                 self.server_id, other.server_id);

        for (i, &other_count) in other.counts.iter().enumerate() {
            let my_count = self.counts[i];
            // Take the MAXIMUM (higher count wins!)
            self.counts[i] = my_count.max(other_count);

            if other_count > my_count {
                println!("  Updated Server {}'s count: {} → {}",
                         i, my_count, other_count);
            }
        }

        println!("✅ Merge complete! New total: {}", self.value());
    }

    /// Show current state
    fn show_state(&self) {
        println!("\n📊 Server {} state:", self.server_id);
        println!("  Counts per server: {:?}", self.counts);
        println!("  Total likes: {}", self.value());
    }
}

/// PN-Counter (Positive-Negative Counter) - can go up AND down
/// Like a bank balance
#[derive(Debug, Clone)]
struct PNCounter {
    server_id: usize,
    // Track increases separately from decreases
    increases: Vec<u32>,
    decreases: Vec<u32>,
}

impl PNCounter {
    fn new(server_id: usize, num_servers: usize) -> Self {
        println!("💰 Created bank balance for Server {}", server_id);
        PNCounter {
            server_id,
            increases: vec![0; num_servers],
            decreases: vec![0; num_servers],
        }
    }

    /// Deposit money (increase)
    fn deposit(&mut self, amount: u32) {
        self.increases[self.server_id] += amount;
        println!("  Server {} deposited ${}", self.server_id, amount);
    }

    /// Withdraw money (decrease)
    fn withdraw(&mut self, amount: u32) {
        self.decreases[self.server_id] += amount;
        println!("  Server {} withdrew ${}", self.server_id, amount);
    }

    /// Get current balance (increases - decreases)
    fn balance(&self) -> i32 {
        let total_increases: u32 = self.increases.iter().sum();
        let total_decreases: u32 = self.decreases.iter().sum();
        total_increases as i32 - total_decreases as i32
    }

    /// Merge with another counter
    fn merge(&mut self, other: &PNCounter) {
        println!("\n🔄 Server {} merging bank data with Server {}...",
                 self.server_id, other.server_id);

        // Merge increases (take maximum)
        for (i, &other_inc) in other.increases.iter().enumerate() {
            self.increases[i] = self.increases[i].max(other_inc);
        }

        // Merge decreases (take maximum)
        for (i, &other_dec) in other.decreases.iter().enumerate() {
            self.decreases[i] = self.decreases[i].max(other_dec);
        }

        println!("✅ Merge complete! New balance: ${}", self.balance());
    }

    fn show_state(&self) {
        println!("\n💰 Server {} bank state:", self.server_id);
        println!("  Deposits: {:?}", self.increases);
        println!("  Withdrawals: {:?}", self.decreases);
        println!("  Balance: ${}", self.balance());
    }
}

fn main() {
    println!("=== CRDT Counter Demo ===");
    println!("(Magic counters that merge automatically!)\n");

    // === Part 1: G-Counter (Likes) ===
    println!("--- Part 1: YouTube-like Like Counter ---");
    println!("Scenario: 3 servers, users adding likes while offline\n");

    // Create 3 servers with G-Counters
    let mut server1 = GCounter::new(0, 3);
    let mut server2 = GCounter::new(1, 3);
    let mut server3 = GCounter::new(2, 3);

    // Users add likes on different servers (offline)
    println!("📱 Users adding likes (servers offline):");
    server1.increment(5);  // 5 likes on server1
    server2.increment(3);  // 3 likes on server2
    server3.increment(7);  // 7 likes on server3

    server1.show_state();
    server2.show_state();
    server3.show_state();

    // Servers reconnect and merge
    println!("\n🌐 Servers reconnect! Time to merge...");
    server1.merge(&server2);
    server1.merge(&server3);

    server1.show_state();
    println!("\n✨ Magic! No conflicts, all {} likes counted!", server1.value());

    // === Part 2: PN-Counter (Bank Balance) ===
    println!("\n\n--- Part 2: Bank Account Balance ---");
    println!("Scenario: 2 ATMs, both processing transactions offline\n");

    let mut atm1 = PNCounter::new(0, 2);
    let mut atm2 = PNCounter::new(1, 2);

    // Transactions happen on both ATMs (offline)
    println!("💳 Transactions happening (ATMs offline):");
    atm1.deposit(100);      // +$100 on ATM1
    atm1.withdraw(20);      // -$20 on ATM1
    atm2.deposit(50);       // +$50 on ATM2
    atm2.withdraw(30);      // -$30 on ATM2

    atm1.show_state();
    atm2.show_state();

    // ATMs reconnect and merge
    println!("\n🌐 ATMs reconnect! Syncing balances...");
    atm1.merge(&atm2);

    atm1.show_state();
    println!("\n✨ Final balance: ${}", atm1.balance());
    println!("   Calculation: (+$100 + $50) - ($20 + $30) = ${}",
             atm1.balance());

    // === Summary ===
    println!("\n\n🎯 Key Points:");
    println!("✓ G-Counter: Only goes UP (perfect for likes, views)");
    println!("✓ PN-Counter: Goes UP and DOWN (perfect for money, scores)");
    println!("✓ No conflicts: Merges happen automatically!");
    println!("✓ Works offline: Each server tracks its own changes");
    println!("✓ Eventually consistent: All servers end up with same total");
    println!("\nWhy ESA needs this:");
    println!("→ Servers can work independently (fast!)");
    println!("→ No errors when merging (reliable!)");
    println!("→ AI always gets correct totals (trustworthy!)");
}
