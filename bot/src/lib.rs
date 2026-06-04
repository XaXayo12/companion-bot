//! Library surface for the AFK Companion bot.
//!
//! The binary (`main.rs`) is a thin wrapper around these modules. Exposing them
//! as a library lets integration tests in `tests/` exercise the WebSocket
//! bridge and the command protocol end-to-end.

pub mod bot;
pub mod bridge;
pub mod config;
pub mod console;
pub mod shared;
