//! # Textcamp!
//!
//! Textcamp is a system for describing and interacting with virtual text worlds.
//!
//! It has two major components: an Actix runtime (see `src/bin/server.rs`) and the
//! core library (see `src/lib.rs`).
//!
//! The runtime is responsible for handling client connections (via websockets),
//! triggering timing events, and delivering messages between the different actors.
//! It relies on `Actix` for most of the functionality.
//!
//! The core library is centered around `src/core/world.rs`, which handles inbound
//! commands and manages events and state within the game.
//!
//! ## Design
//!
//! [World](textcamp::core::world::World) contains the shared state of the game.
//!
//! The [Connection] actor handles the client websocket connections, and messages
//! passing to and from a shared instance of [World].
//!
//! All spaces, mobs, and items are described in templates. You can find some examples
//! in the `world/` directory.

// (add missing_docs to warn)
#![warn(rust_2018_idioms)]

/// Actix Actors for the runtime
pub mod actors;

/// Core library for state and game play
pub mod core;

/// For reading templates describing spaces, mobs, and items
pub mod templates;

/// Database and email services
pub mod services;

// TODO: put me in utils or something, use for command verbs, skills, etc
fn normalize_str(input: &str) -> String {
    input.trim().to_uppercase()
}
