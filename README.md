# ESA - Executable State Architecture

> AI-Governed Autonomous Distributed State Runtime

## Project Status: Day 2 Complete ✅

**Current Phase:** Phase 1 - Problem Definition and Literature Review  
**Day:** 2 of 365  
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
- **Storage:** RocksDB (planned)
- **WASM Runtime:** Wasmtime (planned)
- **SLM:** TinyLlama/Phi/Qwen (to be selected)
- **Serialization:** Serde
- **Logging:** Tracing + tracing-subscriber
- **Orchestration:** Kubernetes (local: kind)
- **Observability:** OpenTelemetry + Prometheus + Grafana (planned)

## Current Progress

### ✅ Completed (Days 1-2)

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

### � Next Steps (Days 3-7)
- [ ] Study distributed systems (CAP theorem, PACELC, Raft, CRDTs)
- [ ] Research storage systems (LSM trees, MVCC, RocksDB)
- [ ] Document findings from distributed systems study
- [ ] Complete Week 1 literature review

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

- [Day-wise Implementation Plan](../ESA_DAYWISE_IMPLEMENTATION_PLAN.md) - Complete 365-day breakdown
- [Week-by-Week Tasks](../WEEK_BY_WEEK_DETAILED_TASKS.md) - Detailed weekly task lists
- [Project Structure](../PROJECT_STRUCTURE.md) - Repository organization
- [Milestone Tracker](../MILESTONE_TRACKER.md) - Progress tracking

## Contributing

This is a research project. For questions or collaboration inquiries, please open an issue on GitHub.

## License

MIT License

## Contact

- **GitHub:** [@sujithputta02](https://github.com/sujithputta02)
- **Repository:** [Esa_db](https://github.com/sujithputta02/Esa_db)

---

**Last Updated:** Day 2 (June 15, 2026)  
**Next Milestone:** Distributed systems study (Days 3-5)  
**Status:** ✅ Environment setup complete, ready for research phase
