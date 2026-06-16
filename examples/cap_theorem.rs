// CAP Theorem Example - The Magic Notebook
// This shows how we choose Consistency + Partition Tolerance for ESA

/// A notebook that follows CAP theorem rules
/// ESA chooses CP (Consistency + Partition tolerance)
struct Notebook {
    name: String,
    data: String,
    // Can we talk to other notebooks?
    can_communicate: bool,
}

impl Notebook {
    /// Create a new notebook
    fn new(name: &str) -> Self {
        println!("📓 Created notebook: {}", name);
        Notebook {
            name: name.to_string(),
            data: String::new(),
            can_communicate: true,
        }
    }

    /// Write something in the notebook
    /// ESA Rule: Only write if we can ensure consistency!
    fn write(&mut self, text: &str) -> Result<(), String> {
        println!("\n✍️  {} wants to write: '{}'", self.name, text);

        // Check: Can we talk to other notebooks to stay consistent?
        if !self.can_communicate() {
            println!("❌ {}: Can't communicate! Waiting to ensure consistency...", self.name);
            println!("   (This is the tradeoff: We wait instead of giving wrong data)");
            return Err("Network partition - waiting for consistency".to_string());
        }

        // All good! Write the data
        self.data = text.to_string();
        println!("✅ {}: Successfully wrote '{}'", self.name, text);
        Ok(())
    }

    /// Read what's in the notebook
    fn read(&self) -> &str {
        println!("📖 {} reads: '{}'", self.name, self.data);
        &self.data
    }

    /// Check if we can communicate (simulated)
    fn can_communicate(&self) -> bool {
        self.can_communicate
    }

    /// Simulate network failure
    fn disconnect(&mut self) {
        println!("🔌 {} disconnected from network!", self.name);
        self.can_communicate = false;
    }

    /// Simulate network recovery
    fn reconnect(&mut self) {
        println!("🔌 {} reconnected to network!", self.name);
        self.can_communicate = true;
    }
}

fn main() {
    println!("=== CAP Theorem Demo ===");
    println!("ESA chooses: Consistency + Partition Tolerance");
    println!("(We prefer to wait rather than give wrong data to AI)\n");

    // Create 3 notebooks (like 3 servers)
    let mut notebook1 = Notebook::new("Server1");
    let mut notebook2 = Notebook::new("Server2");
    let mut notebook3 = Notebook::new("Server3");

    // Scenario 1: Everything working normally
    println!("\n--- Scenario 1: Normal Operation ---");
    notebook1.write("Hello World").unwrap();
    notebook2.write("Hello World").unwrap();
    notebook3.write("Hello World").unwrap();
    println!("✓ All notebooks have: '{}'", notebook1.read());

    // Scenario 2: Network partition!
    println!("\n--- Scenario 2: Network Partition ---");
    notebook2.disconnect();
    
    // Notebook 2 can't write (ensures consistency)
    match notebook2.write("Emergency Update") {
        Ok(_) => println!("Wrote data"),
        Err(e) => println!("⏸️  Paused operation: {}", e),
    }

    // Notebook 1 and 3 can still write (partition tolerant!)
    notebook1.write("Update from Server1").unwrap();
    notebook3.write("Update from Server1").unwrap();

    // Scenario 3: Network recovers
    println!("\n--- Scenario 3: Network Recovery ---");
    notebook2.reconnect();
    notebook2.write("Synced Update").unwrap();
    
    println!("\n🎯 Key Points:");
    println!("✓ We chose CONSISTENCY: All notebooks match");
    println!("✓ We chose PARTITION TOLERANCE: Works even with network failures");
    println!("✗ We sacrificed AVAILABILITY: Server2 waited during partition");
    println!("\nWhy? Because AI needs CORRECT data to make good decisions!");
}
