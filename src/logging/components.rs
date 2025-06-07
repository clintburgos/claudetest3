use bevy::prelude::*;
use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub frame: u32,
    pub category: LogCategory,
    pub message: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogCategory {
    Keypress,
    MouseClick,
    MouseMove,
    GameEvent,
    SystemEvent,
    PerformanceMetric,
    StateChange,
    Screenshot,
    Custom(String),
}

#[derive(Resource)]
pub struct LogBuffer {
    pub entries: VecDeque<LogEntry>,
    pub max_entries: usize,
    pub current_frame: u32,
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new(10000) // Keep last 10k entries in memory
    }
}

impl LogBuffer {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
            current_frame: 0,
        }
    }

    pub fn add_entry(&mut self, category: LogCategory, message: String, data: Option<String>) {
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            frame: self.current_frame,
            category,
            message,
            data,
        };

        self.entries.push_back(entry);

        // Remove old entries if buffer is full
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn increment_frame(&mut self) {
        self.current_frame += 1;
    }
}

#[derive(Event)]
pub struct LogEvent {
    pub category: LogCategory,
    pub message: String,
    pub data: Option<String>,
}
