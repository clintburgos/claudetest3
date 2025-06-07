use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use std::time::SystemTime;

use crate::logging::components::*;
use crate::logging::writer::LogWriter;

#[derive(Resource)]
pub struct ScreenshotTimer {
    timer: Timer,
}

impl Default for ScreenshotTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

pub fn capture_screenshots(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ScreenshotTimer>,
    log_writer: Res<LogWriter>,
    mut log_events: EventWriter<LogEvent>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let screenshot_path = log_writer.get_screenshot_path(timestamp);
        
        // Request screenshot using the observer pattern
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(screenshot_path.clone()));

        // Log the screenshot event
        log_events.write(LogEvent {
            category: LogCategory::Screenshot,
            message: "Screenshot captured".to_string(),
            data: Some(format!("path: {}", screenshot_path.display())),
        });
    }
}