# ESA - Executable State Architecture

> AI-Governed Autonomous Distributed State Runtime

## Project Status: Day 1 - Initial Setup

**Current Phase:** Phase 1 - Problem Definition and Literature Review  
**Day:** 1 of 365  
**Started:** June 15, 2026

## Overview

ESA is a research prototype for a self-organizing distributed runtime where:
- State is executable, not only stored
- AI assists orchestration under hard safety rules
- Developers express intent instead of low-level commands
- Security and policy enforcement are built-in from the start

## Architecture (In Progress)

The system will consist of 6 layers:
1. **Active Entity Runtime** - Executable state entities with WASM sandbox
2. **Distributed State Fabric** - Raft-based replication and partitioning
3. **AI Governance Layer** - SLM-powered orchestration
4. **Deterministic Safety Layer** - Policy enforcement and action gateway
5. **Intent Engine** - High-level developer interface
6. **Observability Layer** - Metrics, tracing, and audit logs

## Technology Stack

- **Language:** Rust (1.96.0)
- **Async Runtime:** Tokio
- **Messaging:** NATS
- **Consensus:** Raft
- **Storage:** RocksDB
- **WASM Runtime:** Wasmtime
- **SLM:** TinyLlama/Phi/Qwen (to be selected)
- **Observability:** OpenTelemetry + Prometheus + Grafana

## Current Progress

### ✅ Completed (Day 1)
- [x] Rust toolchain installed (v1.96.0)
- [x] Cargo tools installed (cargo-watch, cargo-edit, cargo-audit)
- [x] Git repository initialized
- [x] Basic project structure created
- [x] .gitignore configured

### 🔄 In Progress
- [ ] Docker and Kubernetes setup
- [ ] Documentation structure
- [ ] Literature review

### 📅 Next Steps (Days 2-7)
- Complete container infrastructure setup
- Begin distributed systems study (CAP, PACELC, Raft)
- Study storage systems (LSM trees, MVCC)
- Set up documentation framework

## Project Timeline

- **Phase 1:** Problem Definition (Month 1 - Days 1-30)
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

## Getting Started

### Prerequisites
- Rust 1.75+ (currently using 1.96.0)
- Docker Desktop
- Kubernetes (kind or minikube)
- NATS server

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd esa-runtime

# Build the project
cargo build

# Run tests
cargo test

# Watch mode for development
cargo watch -x check -x test
```

## Documentation

- [Day-wise Implementation Plan](../ESA_DAYWISE_IMPLEMENTATION_PLAN.md)
- [Week-by-Week Tasks](../WEEK_BY_WEEK_DETAILED_TASKS.md)
- [Milestone Tracker](../MILESTONE_TRACKER.md)

(More documentation will be added as the project progresses)

## Contributing

This is a research project. Contributions, suggestions, and discussions are welcome once the core architecture is established.

## License

To be determined

## Contact

Project maintained as part of final year research project.

---

**Last Updated:** Day 1 (June 15, 2026)  
**Next Milestone:** Complete development environment setup (Day 2)
