// ESA Runtime - Executable State Architecture
// Main library file organizing all modules

pub mod governance;  // SLM governance and constitutional AI
pub mod safety;      // Policy engine and safety layer
pub mod runtime;     // Distributed runtime core (NATS, Raft, consensus)
pub mod entity;      // Executable state entities with WASM
pub mod intent;      // Intent DSL and policy mapping
pub mod storage;     // Persistent storage layer
pub mod network;     // Network communication primitives
pub mod monitoring;  // Observability and metrics

// Re-export commonly used types
pub use governance::*;
pub use safety::*;
pub use runtime::*;
pub use entity::*;
