# ESA - Executable State Architecture

> AI-Governed Autonomous Distributed State Runtime

## Project Status: Day 6 Complete ✅

**Current Phase:** Phase 1 - Problem Definition and Literature Review  
**Day:** 6 of 365  
**Started:** June 15, 2026

## Overview

ESA is a research prototype for a self-organizing distributed runtime where:
- State is executable, not only stored
- AI assists orchestration under hard safety rules
- Developers express intent instead of low-level commands
- Security and policy enforcement are built-in from the start

## Architecture

The system consists of 6 layers:

```
┌─────────────────────────────────────────┐
│   Intent Engine (DSL → Policy)          │
├─────────────────────────────────────────┤
│   SLM Governance (Constitutional AI)    │
├─────────────────────────────────────────┤
│   Safety Layer (Policy Enforcement)     │
├─────────────────────────────────────────┤
│   Executable State Entities (WASM)      │
├─────────────────────────────────────────┤
│   Distributed Runtime (Raft + NATS)     │
├─────────────────────────────────────────┤
│   Storage Layer                          │
└─────────────────────────────────────────┘
```

## Project Structure

```
src/
├── governance/       # SLM orchestration and constitutional AI
│   ├── slm.rs
│   ├── constitution.rs
│   └── workload.rs
├── safety/          # Deterministic safety and policy engine
│   ├── policy.rs
│   ├── gateway.rs
│   └── audit.rs
├── runtime/         # Distributed runtime core
│   ├── messaging.rs    # NATS pub/sub
│   ├── consensus.rs    # Raft implementation
│   ├── replication.rs
│   └── membership.rs
├── entity/          # Executable state entities
│   ├── schema.rs
│   ├── lifecycle.rs
│   └── executor.rs     # WASM runtime
├── intent/          # Intent DSL and compiler
│   ├── dsl.rs
│   ├── parser.rs
│   └── compiler.rs
├── storage/         # Persistent storage layer
│   ├── engine.rs
│   └── index.rs
├── network/         # Network primitives
│   ├── transport.rs
│   └── protocol.rs
└── monitoring/      # Observability
    ├── metrics.rs
    └── health.rs
```

## Technology Stack

- **Language:** Rust 2021 edition (1.96.0)
- **Async Runtime:** Tokio
- **Messaging:** NATS (async-nats)
- **Consensus:** Raft (planned)
- **Storage:** RocksDB (0.22)
- **WASM Runtime:** Wasmtime (planned)
- **SLM:** TinyLlama/Phi/Qwen (to be selected)
- **Serialization:** Serde
- **Logging:** Tracing + tracing-subscriber
- **Orchestration:** Kubernetes (local: kind)
- **Observability:** OpenTelemetry + Prometheus + Grafana (planned)

## Current Progress

### ✅ Completed (Days 1-6)

**Day 1:**
- [x] Rust toolchain installed (v1.96.0)
- [x] Cargo tools installed (cargo-watch, cargo-edit, cargo-audit)
- [x] Git repository initialized and connected to GitHub
- [x] Basic project structure created
- [x] .gitignore configured

**Day 2:**
- [x] kubectl installed (v1.36.2)
- [x] kind installed (v0.32.0)
- [x] Local Kubernetes cluster created (esa-dev)
- [x] NATS server running in Docker (ports 4222, 8222)
- [x] Complete project structure with 8 core modules
- [x] Cargo.toml configured with async dependencies
- [x] lib.rs with module organization
- [x] main.rs with async runtime and tracing
- [x] Project compiles successfully (`cargo check` passes)

**Day 3:**
- [x] Studied CAP theorem and PACELC
- [x] Researched Raft consensus algorithm
- [x] Learned CRDTs (Conflict-free Replicated Data Types)
- [x] Studied consistent hashing and partitioning strategies
- [x] Created comprehensive documentation (`docs/day3_distributed_systems.md`)
- [x] Built 4 working example programs:
  - `examples/cap_theorem.rs` - CAP theorem demonstration
  - `examples/raft_election.rs` - Leader election simulation
  - `examples/crdt_counter.rs` - CRDT G-Counter and PN-Counter
  - `examples/consistent_hashing.rs` - Data distribution demo
- [x] All examples compile and run successfully

**Day 4:**
- [x] Deep dive into Raft consensus algorithm details
- [x] Studied Raft roles (Leader, Follower, Candidate)
- [x] Learned log replication workflow and commit process
- [x] Understood heartbeat mechanism and failure detection
- [x] Studied all 5 Raft safety guarantees
- [x] Researched data partitioning strategies (Range, Hash, Consistent Hashing)
- [x] Learned rebalancing when adding/removing servers
- [x] Created detailed documentation (`docs/day4_raft_and_partitioning.md`)
- [x] Built 3 working example programs:
  - `examples/raft_log_replication.rs` - Leader-follower log replication
  - `examples/partitioning_strategies.rs` - Compare partitioning methods
  - `examples/rebalancing_demo.rs` - Data movement when servers change
- [x] All examples compile and run successfully

**Day 5:**
- [x] Studied LSM tree architecture (MemTable, SSTable, Compaction)
- [x] Researched RocksDB architecture and features
- [x] Learned MVCC (Multi-Version Concurrency Control)
- [x] Studied Write-Ahead Logging (WAL) for durability
- [x] Created comprehensive documentation (`docs/day5_storage_systems.md`)
- [x] Designed ESA storage layer architecture
- [x] Built working example program:
  - `examples/storage_basics.rs` - LSM tree, MVCC, Column Families simulation
- [x] Created automated test script (`test_examples.sh`)
- [x] All 8 examples compile without warnings

**Day 6:**
- [x] Explored RocksDB Rust bindings (rust-rocksdb)
- [x] Added RocksDB dependency to Cargo.toml
- [x] Studied Column Families in depth
- [x] Designed ESA Column Family structure (entities, metadata, raft_log, snapshots)
- [x] Researched backup/restore strategies (Checkpoint, Backup Engine, Manual)
- [x] Learned Bloom filters and prefix scans
- [x] Created comprehensive documentation (`docs/day6_rocksdb_integration.md`)
- [x] Created day summary (`docs/day6_summary.md`)
- [x] Built proof-of-concept example:
  - `examples/rocksdb_poc.rs` - Complete RocksDB integration demo
- [x] Documented performance tuning guidelines

### 🔄 Next Steps (Day 7)
- [ ] Review all Week 1 learnings
- [ ] Organize documentation and create mind maps
- [ ] Write Week 1 progress report
- [ ] Plan Week 2 literature review
- [ ] Set up project tracking (GitHub Projects)

## Getting Started

### Prerequisites
- Rust 1.96.0 or later
- Docker Desktop
- kubectl 1.36.2 or later
- kind 0.32.0 or later

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/sujithputta02/Esa_db.git
   cd esa-runtime
   ```

2. **Start local infrastructure**
   ```bash
   # Start Kubernetes cluster
   kind create cluster --name esa-dev
   
   # Start NATS server
   docker run -d --name nats-main -p 4222:4222 -p 8222:8222 nats:latest
   ```

3. **Build and run**
   ```bash
   # Build the project
   cargo build
   
   # Run the runtime
   cargo run
   
   # Run tests
   cargo test
   ```

### Development Tools

```bash
# Install development tools (if not already installed)
cargo install cargo-watch cargo-edit cargo-audit

# Auto-rebuild on changes
cargo watch -x check -x test

# Check for security vulnerabilities
cargo audit
```

### Running Examples

Days 3-6 include educational examples demonstrating distributed systems and storage concepts:

**Day 3 Examples - Distributed Systems:**
```bash
# CAP Theorem - Shows ESA's CP (Consistency + Partition Tolerance) choice
cargo run --example cap_theorem

# Raft Consensus - Simulates leader election with 5 servers
cargo run --example raft_election

# CRDTs - Demonstrates conflict-free counters (G-Counter and PN-Counter)
cargo run --example crdt_counter

# Consistent Hashing - Shows smart data distribution across servers
cargo run --example consistent_hashing
```

**Day 4 Examples - Raft & Partitioning:**
```bash
# Raft Log Replication - Leader writes, followers copy, commit with majority
cargo run --example raft_log_replication

# Partitioning Strategies - Compare Range, Hash, and Consistent Hashing
cargo run --example partitioning_strategies

# Rebalancing Demo - Shows data movement when servers are added/removed
cargo run --example rebalancing_demo
```

**Day 5 Example - Storage Systems:**
```bash
# Storage Basics - LSM tree, MVCC, and Column Families simulation
cargo run --example storage_basics
```

**Day 6 Example - RocksDB Integration:**
```bash
# RocksDB POC - Complete integration demo with all features
# Note: First build may take 5-10 minutes (compiling RocksDB)
cargo run --example rocksdb_poc
```

**Test All Examples:**
```bash
# Automated test script for all examples
./test_examples.sh
```

Each example includes:
- Kid-friendly explanations with real-world analogies
- Print statements showing what's happening step-by-step
- Simple variable names and clear comments
- Direct relevance to ESA's architecture

## Verify Setup

Check that all infrastructure is running:

```bash
# Check Kubernetes cluster
kubectl cluster-info --context kind-esa-dev

# Check NATS
docker ps | grep nats

# Check project builds
cargo check
```

## Project Timeline

- **Phase 1:** Problem Definition (Month 1 - Days 1-30)
  - Week 1: Foundation & Setup ← **Current**
- **Phase 2:** SLM Governance Core (Months 2-3 - Days 31-90)
- **Phase 3:** Safety Layer (Month 4 - Days 91-120)
- **Phase 4:** Distributed Runtime (Months 5-6 - Days 121-180)
- **Phase 5:** Entity Framework (Month 7 - Days 181-210)
- **Phase 6:** Intent Engine (Month 8 - Days 211-240)
- **Phase 7:** SLM Integration (Months 9-10 - Days 241-300)
- **Phase 8:** Benchmarking (Month 11 - Days 301-330)
- **Phase 9:** Publication (Month 12 - Days 331-365)

## Research Goals

This project aims to answer:
1. Can a small language model directly execute bounded cluster actions safely?
2. Does executable state metadata improve locality and replication decisions?
3. Can constitutional guardrails reduce harmful orchestration actions?
4. How much can token cost be reduced using optimization techniques?

## Documentation

- [Implementation Plan](../ESA_DAYWISE_IMPLEMENTATION_PLAN.md) - Complete 365-day breakdown
- [Weekly Tasks](../WEEK_BY_WEEK_DETAILED_TASKS.md) - Detailed weekly task lists
- [Project Structure](../PROJECT_STRUCTURE.md) - Repository organization
- [Milestone Tracker](../MILESTONE_TRACKER.md) - Progress tracking

**Learning Documentation:**
- [Day 3: Distributed Systems](docs/day3_distributed_systems.md) - Kid-friendly guide to CAP, Raft, CRDTs, and Consistent Hashing
- [Day 4: Raft & Partitioning](docs/day4_raft_and_partitioning.md) - Deep dive into Raft consensus, partitioning strategies, and rebalancing
- [Day 5: Storage Systems](docs/day5_storage_systems.md) - Comprehensive guide to LSM trees, RocksDB, MVCC, and WAL
- [Day 6: RocksDB Integration](docs/day6_rocksdb_integration.md) - Complete RocksDB integration guide with examples
- [Day 6: Summary](docs/day6_summary.md) - Quick reference for Day 6 learnings

## Contributing

This is a research project. For questions or collaboration inquiries, please open an issue on GitHub.

## License

MIT License

## Contact

- **GitHub:** [@sujithputta02](https://github.com/sujithputta02)
- **Repository:** [Esa_db](https://github.com/sujithputta02/Esa_db)

---

**Last Updated:** Day 6 (July 5, 2026)  
**Next Milestone:** Week 1 Review & Planning (Day 7)  
**Status:** ✅ Week 1 nearly complete - foundational distributed systems and storage concepts mastered
