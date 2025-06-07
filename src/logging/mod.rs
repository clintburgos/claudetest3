pub mod components;
pub mod screenshot;
pub mod systems;
pub mod writer;

pub use components::*;
pub use screenshot::*;
pub use systems::*;
pub use writer::*;

use bevy::prelude::*;

pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LogWriter>()
            .init_resource::<LogBuffer>()
            .init_resource::<ScreenshotTimer>()
            .add_event::<LogEvent>()
            .add_systems(Startup, setup_logging)
            .add_systems(
                Update,
                (
                    log_keypresses,
                    log_mouse_events,
                    log_game_events,
                    capture_screenshots,
                    write_logs_to_file,
                )
                    .chain(),
            );
    }
}
