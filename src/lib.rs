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
//! The [Connection] actors handle the client websocket connections, and messages
//! passing to and from a shared instance of [World].
//!
//! The [Periodic] actor triggers two kinds of events in [World]: the `Tick` event,
//! and the `Melee` event.
//!
//! `Tick` events represent the coarse passage of time. It updates the world clock, and
//! propagates through all of the spaces, mobs, and items. Spaces can spawn mobs and items
//! during their tick, and modify the state of anything in side of them.
//!
//! `Melee` events are triggered every second, to make combat more responsive.
//!
//! All spaces, mobs, and items are described in templates. You can find some examples
//! in the `world/` directory.
//!
//! (add missing_docs to warn)
#![warn(missing_debug_implementations, rust_2018_idioms)]

/// Actix Actors for the runtime
pub mod actors;

/// Core library for state and game play
pub mod core;

/// For reading templates describing spaces, mobs, and items
pub mod templates;

/// Database and email services
pub mod services;
