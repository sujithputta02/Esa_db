/// Day 4 Example: Raft Log Replication
/// 
/// This example shows how Raft replicates log entries across servers.
/// We'll simulate a leader writing data and followers copying it.
/// 
/// Real-world analogy: Project leader writes in notebook, others copy!

#[derive(Debug, Clone, PartialEq)]
enum ServerRole {
    Leader,
    Follower,
}

#[derive(Debug, Clone)]
struct LogEntry {
    index: usize,
    term: u64,
    command: String,
    committed: bool,
}

struct RaftServer {
    id: String,
    _role: ServerRole, // Prefixed with _ to indicate intentionally unused in this demo
    term: u64,
    log: Vec<LogEntry>,
    commit_index: usize,
}

impl RaftServer {
    fn new(id: &str, role: ServerRole) -> Self {
        RaftServer {
            id: id.to_string(),
            _role: role,
            term: 1,
            log: Vec::new(),
            commit_index: 0,
        }
    }

    fn leader_append_entry(&mut self, command: String) -> LogEntry {
        let new_entry = LogEntry {
            index: self.log.len() + 1,
            term: self.term,
            command,
            committed: false,
        };
        self.log.push(new_entry.clone());
        println!("📝 [{}] Leader wrote entry #{}: {}", 
                 self.id, new_entry.index, new_entry.command);
        new_entry
    }

    fn follower_replicate_entry(&mut self, entry: LogEntry) -> bool {
        // Simulate replication delay
        println!("   📨 [{}] Received entry #{}: {}", 
                 self.id, entry.index, entry.command);
        
        self.log.push(entry);
        println!("   ✅ [{}] Replicated entry #{}", self.id, self.log.len());
        true
    }

    fn commit_entry(&mut self, index: usize) {
        if index <= self.log.len() && index > self.commit_index {
            for i in self.commit_index..index {
                self.log[i].committed = true;
            }
            self.commit_index = index;
            println!("🎯 [{}] Committed entries up to #{}", self.id, index);
        }
    }

    fn print_log(&self) {
        println!("\n📋 [{}] Log State (committed up to #{}):", self.id, self.commit_index);
        for entry in &self.log {
            let status = if entry.committed { "✅ committed" } else { "⏳ pending" };
            println!("   [{}] Term {}: {} - {}", 
                     entry.index, entry.term, entry.command, status);
        }
    }
}

fn main() {
    println!("🚀 Day 4 Example: Raft Log Replication");
    println!("======================================\n");

    println!("📚 Concept: Leader writes, followers copy, then commit!\n");

    // Create a cluster with 1 leader and 4 followers
    let mut leader = RaftServer::new("Server-1 (Leader)", ServerRole::Leader);
    let mut follower2 = RaftServer::new("Server-2", ServerRole::Follower);
    let mut follower3 = RaftServer::new("Server-3", ServerRole::Follower);
    let mut follower4 = RaftServer::new("Server-4", ServerRole::Follower);
    let mut follower5 = RaftServer::new("Server-5", ServerRole::Follower);

    println!("📊 Cluster Setup: 1 Leader + 4 Followers");
    println!("   Majority needed: 3 out of 5 servers\n");

    // Scenario 1: Client sends first write request
    println!("─────────────────────────────────────");
    println!("📝 CLIENT REQUEST #1: Write 'user_123 = active'");
    println!("─────────────────────────────────────\n");

    let entry1 = leader.leader_append_entry("user_123 = active".to_string());
    
    println!("\n📢 Leader replicating to followers...\n");
    let ack2 = follower2.follower_replicate_entry(entry1.clone());
    let ack3 = follower3.follower_replicate_entry(entry1.clone());
    let ack4 = follower4.follower_replicate_entry(entry1.clone());
    let ack5 = follower5.follower_replicate_entry(entry1.clone());

    let acks = [ack2, ack3, ack4, ack5].iter().filter(|&&x| x).count();
    println!("\n🎯 Leader received {} acknowledgments (need 2 for majority)", acks);
    
    if acks >= 2 {
        println!("✅ MAJORITY ACHIEVED! (2/4 followers + leader = 3/5 total)");
        leader.commit_entry(1);
        follower2.commit_entry(1);
        follower3.commit_entry(1);
        follower4.commit_entry(1);
        follower5.commit_entry(1);
    }

    leader.print_log();

    // Scenario 2: Client sends second write request
    println!("\n\n─────────────────────────────────────");
    println!("📝 CLIENT REQUEST #2: Write 'user_456 = inactive'");
    println!("─────────────────────────────────────\n");

    let entry2 = leader.leader_append_entry("user_456 = inactive".to_string());
    
    println!("\n📢 Leader replicating to followers...\n");
    follower2.follower_replicate_entry(entry2.clone());
    follower3.follower_replicate_entry(entry2.clone());
    follower4.follower_replicate_entry(entry2.clone());
    // Simulate: follower5 is slow and hasn't responded yet
    println!("   ⏰ [Server-5] Still processing... (slow network)");

    println!("\n🎯 Leader received 3 acknowledgments (need 2 for majority)");
    println!("✅ MAJORITY ACHIEVED! (3/4 responded + leader = 4/5 total)");
    
    leader.commit_entry(2);
    follower2.commit_entry(2);
    follower3.commit_entry(2);
    follower4.commit_entry(2);
    // follower5 will catch up later

    leader.print_log();
    follower2.print_log();

    // Scenario 3: Slow follower catches up
    println!("\n\n─────────────────────────────────────");
    println!("🔄 Follower 5 catches up!");
    println!("─────────────────────────────────────\n");

    follower5.follower_replicate_entry(entry2);
    follower5.commit_entry(2);
    follower5.print_log();

    // Show final state
    println!("\n\n┌────────────────────────────────────┐");
    println!("│  ✅ REPLICATION COMPLETE!         │");
    println!("└────────────────────────────────────┘");
    println!("\n📊 Final State:");
    println!("   • All 5 servers have identical logs");
    println!("   • 2 entries committed across the cluster");
    println!("   • System is consistent and durable");
    
    println!("\n💡 Key Learnings:");
    println!("   1. Leader writes first, then replicates");
    println!("   2. Need majority (3/5) to commit");
    println!("   3. Slow servers catch up eventually");
    println!("   4. Once committed, data is SAFE!");
    
    println!("\n🎯 How ESA Uses This:");
    println!("   • Every state change goes through Raft");
    println!("   • AI decisions are logged and replicated");
    println!("   • Majority ensures no data loss");
    println!("   • Audit trail of all AI actions!");
}
