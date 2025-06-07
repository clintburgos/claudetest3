use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::logging::components::*;
use crate::logging::writer::LogWriter;

pub fn setup_logging(log_writer: Res<LogWriter>) {
    if let Err(e) = log_writer.write_header() {
        error!("Failed to write log header: {}", e);
    }
}

pub fn log_keypresses(
    mut key_events: EventReader<KeyboardInput>,
    mut log_events: EventWriter<LogEvent>,
) {
    for event in key_events.read() {
        let state = if event.state.is_pressed() {
            "pressed"
        } else {
            "released"
        };

        let key_name = match event.key_code {
            KeyCode::KeyA => "A",
            KeyCode::KeyB => "B",
            KeyCode::KeyC => "C",
            KeyCode::KeyD => "D",
            KeyCode::KeyE => "E",
            KeyCode::KeyF => "F",
            KeyCode::KeyG => "G",
            KeyCode::KeyH => "H",
            KeyCode::KeyI => "I",
            KeyCode::KeyJ => "J",
            KeyCode::KeyK => "K",
            KeyCode::KeyL => "L",
            KeyCode::KeyM => "M",
            KeyCode::KeyN => "N",
            KeyCode::KeyO => "O",
            KeyCode::KeyP => "P",
            KeyCode::KeyQ => "Q",
            KeyCode::KeyR => "R",
            KeyCode::KeyS => "S",
            KeyCode::KeyT => "T",
            KeyCode::KeyU => "U",
            KeyCode::KeyV => "V",
            KeyCode::KeyW => "W",
            KeyCode::KeyX => "X",
            KeyCode::KeyY => "Y",
            KeyCode::KeyZ => "Z",
            KeyCode::Digit0 => "0",
            KeyCode::Digit1 => "1",
            KeyCode::Digit2 => "2",
            KeyCode::Digit3 => "3",
            KeyCode::Digit4 => "4",
            KeyCode::Digit5 => "5",
            KeyCode::Digit6 => "6",
            KeyCode::Digit7 => "7",
            KeyCode::Digit8 => "8",
            KeyCode::Digit9 => "9",
            KeyCode::Space => "Space",
            KeyCode::Enter => "Enter",
            KeyCode::Escape => "Escape",
            KeyCode::Backspace => "Backspace",
            KeyCode::Tab => "Tab",
            KeyCode::ShiftLeft => "LeftShift",
            KeyCode::ShiftRight => "RightShift",
            KeyCode::ControlLeft => "LeftCtrl",
            KeyCode::ControlRight => "RightCtrl",
            KeyCode::AltLeft => "LeftAlt",
            KeyCode::AltRight => "RightAlt",
            KeyCode::ArrowUp => "Up",
            KeyCode::ArrowDown => "Down",
            KeyCode::ArrowLeft => "Left",
            KeyCode::ArrowRight => "Right",
            _ => "Unknown",
        };

        log_events.write(LogEvent {
            category: LogCategory::Keypress,
            message: format!("Key {} {}", key_name, state),
            data: Some(format!("keycode: {:?}", event.key_code)),
        });
    }
}

pub fn log_mouse_events(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut log_events: EventWriter<LogEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Log mouse button events
    for event in mouse_button_events.read() {
        let state = if event.state.is_pressed() {
            "pressed"
        } else {
            "released"
        };
        let button = match event.button {
            MouseButton::Left => "Left",
            MouseButton::Right => "Right",
            MouseButton::Middle => "Middle",
            MouseButton::Back => "Back",
            MouseButton::Forward => "Forward",
            MouseButton::Other(_) => return,
        };

        // Try to get cursor position
        let cursor_pos = windows
            .single()
            .ok()
            .and_then(|window| window.cursor_position())
            .map(|pos| format!("({:.1}, {:.1})", pos.x, pos.y))
            .unwrap_or_else(|| "unknown".to_string());

        log_events.write(LogEvent {
            category: LogCategory::MouseClick,
            message: format!("{} mouse button {}", button, state),
            data: Some(format!("position: {}", cursor_pos)),
        });
    }

    // Log mouse motion (throttled to avoid spam)
    static mut MOTION_COUNTER: u32 = 0;
    unsafe {
        for event in mouse_motion_events.read() {
            MOTION_COUNTER += 1;
            // Only log every 10th mouse motion event to avoid spam
            if MOTION_COUNTER % 10 == 0 {
                log_events.write(LogEvent {
                    category: LogCategory::MouseMove,
                    message: "Mouse moved".to_string(),
                    data: Some(format!(
                        "delta: ({:.1}, {:.1})",
                        event.delta.x, event.delta.y
                    )),
                });
            }
        }
    }
}

pub fn log_game_events(
    _log_events: EventWriter<LogEvent>,
    // Add queries for game-specific components here
) {
    // This is where you'd log game-specific events
    // For example: state changes, entity spawning, etc.
}

pub fn write_logs_to_file(
    mut log_buffer: ResMut<LogBuffer>,
    mut log_event_reader: EventReader<LogEvent>,
    log_writer: Res<LogWriter>,
    time: Res<Time>,
) {
    // Update frame counter
    log_buffer.increment_frame();

    // Process new log events
    for event in log_event_reader.read() {
        log_buffer.add_entry(
            event.category.clone(),
            event.message.clone(),
            event.data.clone(),
        );

        // Write to file immediately
        if let Some(entry) = log_buffer.entries.back() {
            if let Err(e) = log_writer.write_entry(entry) {
                error!("Failed to write log entry: {}", e);
            }
        }
    }

    // Log performance metrics every second
    static mut PERF_TIMER: f32 = 0.0;
    unsafe {
        PERF_TIMER += time.delta_secs();
        if PERF_TIMER >= 1.0 {
            PERF_TIMER = 0.0;

            let fps = 1.0 / time.delta_secs();
            log_buffer.add_entry(
                LogCategory::PerformanceMetric,
                format!("FPS: {:.1}", fps),
                Some(format!("delta_time: {:.3}ms", time.delta_secs() * 1000.0)),
            );

            // Write performance entry
            if let Some(entry) = log_buffer.entries.back() {
                if let Err(e) = log_writer.write_entry(entry) {
                    error!("Failed to write performance log: {}", e);
                }
            }
        }
    }
}
