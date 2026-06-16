// Raft Consensus - Leader Election (Like Class Monitor!)
// This shows how servers elect ONE leader to coordinate everything

use std::collections::HashMap;

/// A server in the cluster (like a student in class)
#[derive(Debug, Clone)]
struct Server {
    id: u32,
    name: String,
    role: Role,
    votes_received: u32,
    voted_for: Option<u32>,  // Who did this server vote for?
}

/// The role a server can have
#[derive(Debug, Clone, PartialEq)]
enum Role {
    Follower,   // Regular student
    Candidate,  // Running for monitor
    Leader,     // The chosen monitor
}

impl Server {
    /// Create a new server (new student joins class)
    fn new(id: u32, name: &str) -> Self {
        println!("🎒 Student {} (Server{}) joins the class", name, id);
        Server {
            id,
            name: name.to_string(),
            role: Role::Follower,  // Everyone starts as a follower
            votes_received: 0,
            voted_for: None,
        }
    }

    /// Start an election (raise hand to be monitor)
    fn start_election(&mut self) {
        println!("\n🗳️  {} raises hand: 'I want to be class monitor!'", self.name);
        self.role = Role::Candidate;
        self.votes_received = 1;  // Vote for yourself
        self.voted_for = Some(self.id);
    }

    /// Vote for another server
    fn vote(&mut self, candidate_id: u32, candidate_name: &str) -> bool {
        // Can only vote once!
        if self.voted_for.is_some() {
            println!("  {} already voted, can't vote again", self.name);
            return false;
        }

        println!("  {} votes for {}", self.name, candidate_name);
        self.voted_for = Some(candidate_id);
        true
    }

    /// Receive a vote from another server
    fn receive_vote(&mut self) {
        self.votes_received += 1;
        println!("  {} now has {} votes", self.name, self.votes_received);
    }

    /// Check if won the election (got majority)
    fn check_if_won(&mut self, total_servers: u32) -> bool {
        let majority = (total_servers / 2) + 1;
        let won = self.votes_received >= majority;

        if won {
            self.role = Role::Leader;
            println!("\n🎉 {} is now the CLASS MONITOR!", self.name);
            println!("   (Got {} votes out of {} servers, needed {})",
                     self.votes_received, total_servers, majority);
        }

        won
    }

    /// Leader does their job (coordinate actions)
    fn coordinate_action(&self, action: &str) {
        if self.role != Role::Leader {
            println!("❌ {} is not the leader, can't coordinate!", self.name);
            return;
        }

        println!("\n👨‍🏫 Leader {} says: '{}'", self.name, action);
        println!("   Everyone, copy this to your notebook!");
    }

    /// Follower follows leader's instruction
    fn follow_instruction(&self, instruction: &str) {
        if self.role == Role::Leader {
            return;  // Leader doesn't follow, they lead!
        }

        println!("  {} copies: '{}'", self.name, instruction);
    }
}

/// Simulate a cluster election
struct Cluster {
    servers: HashMap<u32, Server>,
}

impl Cluster {
    fn new() -> Self {
        Cluster {
            servers: HashMap::new(),
        }
    }

    /// Add a server to the cluster
    fn add_server(&mut self, server: Server) {
        self.servers.insert(server.id, server);
    }

    /// Run an election
    fn run_election(&mut self, candidate_id: u32) {
        println!("\n=== ELECTION STARTING ===");

        // Candidate starts election
        if let Some(candidate) = self.servers.get_mut(&candidate_id) {
            candidate.start_election();
        }

        let candidate_name = self.servers.get(&candidate_id)
            .map(|s| s.name.clone())
            .unwrap_or_default();

        // Other servers vote
        let server_ids: Vec<u32> = self.servers.keys().copied()
            .filter(|&id| id != candidate_id)
            .collect();

        for id in server_ids {
            if let Some(server) = self.servers.get_mut(&id) {
                // Let's say 70% vote for the candidate (simulated)
                if id % 3 != 0 {  // Simple random-ish voting
                    if server.vote(candidate_id, &candidate_name) {
                        // Candidate receives the vote
                        if let Some(candidate) = self.servers.get_mut(&candidate_id) {
                            candidate.receive_vote();
                        }
                    }
                }
            }
        }

        // Check if candidate won
        let total = self.servers.len() as u32;
        if let Some(candidate) = self.servers.get_mut(&candidate_id) {
            candidate.check_if_won(total);
        }
    }

    /// Leader coordinates an action
    fn leader_coordinate(&self, action: &str) {
        // Find the leader
        let leader = self.servers.values()
            .find(|s| s.role == Role::Leader);

        if let Some(leader) = leader {
            leader.coordinate_action(action);

            // All followers copy the action
            for server in self.servers.values() {
                if server.role != Role::Leader {
                    server.follow_instruction(action);
                }
            }

            println!("\n✅ Action completed! All servers synchronized.");
        } else {
            println!("❌ No leader found! Need to run election first.");
        }
    }

    /// Simulate leader failure
    fn leader_fails(&mut self) {
        println!("\n💥 OH NO! The leader is absent today!");

        // Find and remove leader
        let leader_id = self.servers.iter()
            .find(|(_, s)| s.role == Role::Leader)
            .map(|(id, _)| *id);

        if let Some(id) = leader_id {
            self.servers.remove(&id);
            println!("   Leader is gone. We need a new election!");
        }

        // Reset everyone's votes for new election
        for server in self.servers.values_mut() {
            server.role = Role::Follower;
            server.votes_received = 0;
            server.voted_for = None;
        }
    }
}

fn main() {
    println!("=== Raft Leader Election Demo ===");
    println!("(Think of it as electing a class monitor!)\n");

    // Create a cluster of 5 servers (5 students)
    let mut cluster = Cluster::new();
    cluster.add_server(Server::new(1, "Alice"));
    cluster.add_server(Server::new(2, "Bob"));
    cluster.add_server(Server::new(3, "Carol"));
    cluster.add_server(Server::new(4, "Dave"));
    cluster.add_server(Server::new(5, "Eve"));

    // Scenario 1: First election
    println!("\n--- Scenario 1: First Election ---");
    cluster.run_election(3);  // Carol runs for leader

    // Scenario 2: Leader does some work
    println!("\n--- Scenario 2: Leader Coordinates ---");
    cluster.leader_coordinate("Write 'user_123 = active' to your notebooks");

    // Scenario 3: Leader fails!
    println!("\n--- Scenario 3: Leader Failure ---");
    cluster.leader_fails();

    // Scenario 4: New election
    println!("\n--- Scenario 4: New Election ---");
    cluster.run_election(2);  // Bob runs for leader

    // Scenario 5: New leader works
    println!("\n--- Scenario 5: New Leader Coordinates ---");
    cluster.leader_coordinate("Write 'user_456 = inactive' to your notebooks");

    println!("\n🎯 Key Points:");
    println!("✓ Only ONE leader at a time");
    println!("✓ Leader coordinates all changes");
    println!("✓ If leader fails, elect new one quickly");
    println!("✓ Majority agreement needed for everything");
    println!("\nWhy? This prevents conflicts and keeps ESA data consistent!");
}
