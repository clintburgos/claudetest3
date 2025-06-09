//! Constants - Shared constants used throughout the application
//!
//! This module centralizes all magic numbers and configuration values
//! to improve maintainability and make the codebase more readable.

use bevy::prelude::*;

/// Grid and map related constants
pub mod grid {
    /// Default tile size in world units
    pub const DEFAULT_TILE_SIZE: f32 = 64.0;

    /// Default map width in tiles
    pub const DEFAULT_MAP_WIDTH: i32 = 200;

    /// Default map height in tiles
    pub const DEFAULT_MAP_HEIGHT: i32 = 200;

    /// Border size for water generation (tiles from edge)
    pub const WATER_BORDER_SIZE: i32 = 5;
}

/// Camera related constants
pub mod camera {
    /// Default camera zoom level
    pub const DEFAULT_ZOOM: f32 = 1.0;

    /// Minimum zoom level (zoomed out)
    pub const MIN_ZOOM: f32 = 0.1;

    /// Maximum zoom level (zoomed in)
    pub const MAX_ZOOM: f32 = 5.0;

    /// Default camera movement speed
    pub const DEFAULT_MOVE_SPEED: f32 = 500.0;

    /// Default camera zoom speed
    pub const DEFAULT_ZOOM_SPEED: f32 = 0.1;

    /// Default friction factor for camera movement
    pub const DEFAULT_FRICTION: f32 = 0.9;

    /// Base FPS for friction calculation
    pub const FRICTION_FPS_BASE: f32 = 60.0;

    /// Velocity threshold below which movement stops
    pub const VELOCITY_STOP_THRESHOLD: f32 = 0.01;

    /// Zoom speed multiplier for keyboard controls
    pub const KEYBOARD_ZOOM_MULTIPLIER: f32 = 5.0;

    /// Pixel scroll unit scaling
    pub const PIXEL_SCROLL_SCALE: f32 = 0.01;

    /// Isometric height ratio
    pub const ISOMETRIC_HEIGHT_RATIO: f32 = 0.5;

    /// Padding multiplier for camera bounds
    pub const BOUNDS_PADDING_MULTIPLIER: f32 = 2.0;

    /// Minimum camera scale for safety
    pub const MIN_CAMERA_SCALE: f32 = 0.1;
}

/// UI dimension constants
pub mod ui {
    /// Header height in pixels
    pub const HEADER_HEIGHT: f32 = 60.0;

    /// Sidebar width in pixels
    pub const SIDEBAR_WIDTH: f32 = 200.0;

    /// Standard button height
    pub const BUTTON_HEIGHT: f32 = 40.0;

    /// Standard padding
    pub const PADDING: f32 = 10.0;

    /// Gap between buttons
    pub const BUTTON_GAP: f32 = 10.0;

    /// Info panel constants
    pub mod info_panel {
        /// Panel width
        pub const WIDTH: f32 = 250.0;

        /// Panel height
        pub const HEIGHT: f32 = 300.0;

        /// Panel padding
        pub const PADDING: f32 = 15.0;

        /// Gap between rows
        pub const ROW_GAP: f32 = 10.0;

        /// Title font size
        pub const TITLE_FONT_SIZE: f32 = 24.0;

        /// Standard text font size
        pub const TEXT_FONT_SIZE: f32 = 16.0;

        /// Separator height
        pub const SEPARATOR_HEIGHT: f32 = 2.0;

        /// Separator vertical margin
        pub const SEPARATOR_MARGIN: f32 = 5.0;

        /// Panel background alpha
        pub const BACKGROUND_ALPHA: f32 = 0.9;

        /// Right margin from edge
        pub const RIGHT_MARGIN: f32 = 10.0;

        /// Top margin below top bar
        pub const TOP_MARGIN: f32 = 70.0;
    }

    /// Performance overlay constants
    pub mod performance {
        /// Overlay margins
        pub const MARGIN: f32 = 10.0;

        /// Overlay padding
        pub const PADDING: f32 = 10.0;

        /// Gap between rows
        pub const ROW_GAP: f32 = 5.0;

        /// Font size
        pub const FONT_SIZE: f32 = 14.0;

        /// Overlay background alpha
        pub const BACKGROUND_ALPHA: f32 = 0.8;
    }
}

/// View culling constants
pub mod culling {
    /// Default buffer tiles around visible area
    /// Increased to ensure no visible gaps at screen edges
    pub const DEFAULT_BUFFER_TILES: i32 = 10;

    /// Maximum tiles to spawn per frame
    /// For a 200x200 map (40,000 tiles), we want fast spawning when zoomed out
    /// Increased to 10000 to reduce blank screen issues when zooming out
    pub const DEFAULT_TILES_PER_FRAME: usize = 10000;

    /// Minimum dynamic buffer size
    pub const MIN_DYNAMIC_BUFFER: f32 = 2.0;

    /// Debug log interval in seconds
    pub const DEBUG_LOG_INTERVAL_SECS: u64 = 1;
}

/// World generation constants
pub mod generation {
    /// Default random seed
    pub const DEFAULT_SEED: u64 = 42;

    /// Default noise scale
    pub const DEFAULT_NOISE_SCALE: f64 = 0.05;

    /// Water level threshold
    pub const WATER_LEVEL: f64 = 0.3;

    /// Mountain level threshold
    pub const MOUNTAIN_LEVEL: f64 = 0.7;

    /// Moisture noise scale multiplier
    pub const MOISTURE_SCALE_MULTIPLIER: f64 = 1.5;

    /// Island shape distance factor
    pub const ISLAND_DISTANCE_FACTOR: f64 = 0.8;

    /// Biome thresholds
    pub mod biome {
        /// Desert moisture threshold
        pub const DESERT_MOISTURE: f64 = 0.3;

        /// Forest moisture threshold
        pub const FOREST_MOISTURE: f64 = 0.7;
    }
}

/// Mesh generation constants
pub mod mesh {
    /// Diamond mesh dimensions
    pub const DIAMOND_WIDTH: f32 = 1.0;
    pub const DIAMOND_HEIGHT_RATIO: f32 = 0.5;

    /// Number of vertices in a diamond tile
    pub const DIAMOND_VERTEX_COUNT: usize = 4;

    /// Number of sides in a hexagon
    pub const HEXAGON_SIDES: i32 = 6;

    /// Angle multiplier for hexagon generation (2 * PI)
    pub const HEXAGON_ANGLE_MULTIPLIER: f32 = 2.0;

    /// UV coordinate constants
    pub const UV_MIN: f32 = 0.0;
    pub const UV_MID: f32 = 0.5;
    pub const UV_MAX: f32 = 1.0;

    /// Beveled tile UV inset
    pub const BEVEL_UV_INSET: f32 = 0.2;
}

/// Color constants
pub mod colors {
    use super::*;

    /// UI background colors
    pub const UI_BACKGROUND_DARK: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const UI_BACKGROUND_MEDIUM: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const UI_BACKGROUND_LIGHT: Color = Color::srgb(0.2, 0.2, 0.2);

    /// Button colors
    pub const BUTTON_NORMAL: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BUTTON_HOVERED: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.5, 0.5, 0.5);

    /// Text colors
    pub const TEXT_PRIMARY: Color = Color::srgb(0.8, 0.8, 0.8);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.6, 0.6, 0.6);

    /// Tile selection colors
    pub const TILE_HOVERED: Color = Color::srgba(1.0, 1.0, 0.0, 0.3);
    pub const TILE_SELECTED: Color = Color::srgba(0.0, 1.0, 0.0, 0.5);
}

/// Performance and timing constants
pub mod timing {
    /// Expected frame time for 60 FPS (in seconds)
    pub const EXPECTED_FRAME_TIME: f32 = 0.016;

    /// Movement speed tolerance
    pub const MOVEMENT_SPEED_TOLERANCE: f32 = 10.0;
}
