//! Game Module - Core game state and management
//!
//! This module handles game state transitions, save/load functionality,
//! and overall game flow control.

pub mod state;

pub use state::{GameState, GameStatePlugin};
