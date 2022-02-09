pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// ├─ src
// │  ├─ lib.rs -> registering modules
// │  ├─ entrypoint.rs -> entrypoint to the program
// │  ├─ instruction.rs -> program API, (de)serializing instruction data
// │  ├─ processor.rs -> program logic
// │  ├─ state.rs -> program objects, (de)serializing state
// │  ├─ error.rs -> program specific errors
// ├─ .gitignore
// ├─ Cargo.lock
// ├─ Cargo.toml
// ├─ Xargo.toml
