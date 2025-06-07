use bevy::prelude::*;
use claudetest3::logging::*;
use std::fs;
use std::path::Path;

#[test]
fn test_log_buffer_creation() {
    let buffer = LogBuffer::new(100);
    assert_eq!(buffer.entries.len(), 0);
    assert_eq!(buffer.max_entries, 100);
    assert_eq!(buffer.current_frame, 0);
}

#[test]
fn test_log_buffer_add_entry() {
    let mut buffer = LogBuffer::new(10);

    buffer.add_entry(
        LogCategory::Keypress,
        "Test keypress".to_string(),
        Some("key: A".to_string()),
    );

    assert_eq!(buffer.entries.len(), 1);
    let entry = &buffer.entries[0];
    assert_eq!(entry.message, "Test keypress");
    assert_eq!(entry.data, Some("key: A".to_string()));
    assert!(matches!(entry.category, LogCategory::Keypress));
}

#[test]
fn test_log_buffer_overflow() {
    let mut buffer = LogBuffer::new(3);

    // Add 5 entries to a buffer with max 3
    for i in 0..5 {
        buffer.add_entry(LogCategory::SystemEvent, format!("Event {}", i), None);
    }

    assert_eq!(buffer.entries.len(), 3);
    // Should contain events 2, 3, 4 (oldest removed)
    assert_eq!(buffer.entries[0].message, "Event 2");
    assert_eq!(buffer.entries[1].message, "Event 3");
    assert_eq!(buffer.entries[2].message, "Event 4");
}

#[test]
fn test_log_writer_creation() {
    // Clean up any existing test session
    let test_session = "logs/test_session";
    let _ = fs::remove_dir_all(test_session);

    let writer = LogWriter::new("test_session").expect("Failed to create log writer");
    assert!(Path::new(test_session).exists());
    assert!(Path::new(&format!("{}/log.txt", test_session)).exists());

    // Clean up
    let _ = fs::remove_dir_all(test_session);
}

#[test]
fn test_log_writer_write_entry() {
    let test_session = "logs/test_write_session";
    let _ = fs::remove_dir_all(test_session);

    let writer = LogWriter::new("test_write_session").expect("Failed to create log writer");

    let entry = LogEntry {
        timestamp: std::time::SystemTime::now(),
        frame: 42,
        category: LogCategory::GameEvent,
        message: "Test event".to_string(),
        data: Some("test data".to_string()),
    };

    writer.write_entry(&entry).expect("Failed to write entry");

    // Read the file to verify
    let log_file = format!("{}/log.txt", test_session);
    let contents = fs::read_to_string(&log_file).expect("Failed to read log file");
    assert!(contents.contains("Frame 42"));
    assert!(contents.contains("GAME_EVENT"));
    assert!(contents.contains("Test event"));
    assert!(contents.contains("test data"));

    // Clean up
    let _ = fs::remove_dir_all(test_session);
}

#[test]
fn test_log_categories() {
    let categories = vec![
        (LogCategory::Keypress, "KEYPRESS"),
        (LogCategory::MouseClick, "MOUSE_CLICK"),
        (LogCategory::MouseMove, "MOUSE_MOVE"),
        (LogCategory::GameEvent, "GAME_EVENT"),
        (LogCategory::SystemEvent, "SYSTEM"),
        (LogCategory::PerformanceMetric, "PERFORMANCE"),
        (LogCategory::StateChange, "STATE_CHANGE"),
        (LogCategory::Screenshot, "SCREENSHOT"),
        (LogCategory::Custom("TEST".to_string()), "TEST"),
    ];

    for (category, _expected_str) in categories {
        let mut buffer = LogBuffer::new(10);
        buffer.add_entry(category.clone(), "Test".to_string(), None);
        assert_eq!(buffer.entries[0].category, category);
    }
}

#[test]
fn test_screenshot_path() {
    let test_session = "logs/test_screenshot";
    let _ = fs::remove_dir_all(test_session);

    let writer = LogWriter::new("test_screenshot").expect("Failed to create log writer");
    
    let timestamp: u128 = 1234567890;
    let screenshot_path = writer.get_screenshot_path(timestamp);
    
    assert_eq!(
        screenshot_path,
        Path::new("logs/test_screenshot/screenshot_1234567890.png")
    );

    // Clean up
    let _ = fs::remove_dir_all(test_session);
}