//! UI Panel Components - Shared components for UI panels
//!
//! This module contains components used across different UI panels
//! for identification and state management.

use bevy::prelude::*;

/// Marker component for the root UI node
#[derive(Component)]
pub struct UIRoot;

/// Marker component for the info panel container
#[derive(Component)]
pub struct InfoPanel;

/// Marker component for the top bar container
#[derive(Component)]
pub struct TopBar;

/// Component for dynamic text that displays tile information
#[derive(Component)]
pub struct TileInfoText;

/// Component for text that displays current biome
#[derive(Component)]
pub struct BiomeText;

/// Component for text that displays tile coordinates
#[derive(Component)]
pub struct CoordinatesText;

/// Marker component for game control buttons
#[derive(Component)]
pub struct GameControlButton;

/// Types of game control buttons
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ButtonAction {
    Pause,
    Resume,
    MainMenu,
    Quit,
}
