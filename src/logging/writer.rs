use bevy::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::logging::components::*;

#[derive(Resource)]
pub struct LogWriter {
    writer: Arc<Mutex<BufWriter<File>>>,
    pub log_dir: PathBuf,
}

impl LogWriter {
    pub fn new(session_name: &str) -> std::io::Result<Self> {
        let log_dir = PathBuf::from(format!("logs/{}", session_name));

        // Create session directory
        std::fs::create_dir_all(&log_dir)?;

        let log_file_path = log_dir.join("log.txt");
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_file_path)?;

        let writer = Arc::new(Mutex::new(BufWriter::new(file)));

        Ok(Self { writer, log_dir })
    }

    pub fn write_entry(&self, entry: &LogEntry) -> std::io::Result<()> {
        let mut writer = self.writer.lock().unwrap();

        let timestamp = entry
            .timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let category_str = match &entry.category {
            LogCategory::Keypress => "KEYPRESS",
            LogCategory::MouseClick => "MOUSE_CLICK",
            LogCategory::MouseMove => "MOUSE_MOVE",
            LogCategory::GameEvent => "GAME_EVENT",
            LogCategory::SystemEvent => "SYSTEM",
            LogCategory::PerformanceMetric => "PERFORMANCE",
            LogCategory::StateChange => "STATE_CHANGE",
            LogCategory::Screenshot => "SCREENSHOT",
            LogCategory::Custom(s) => s,
        };

        let data_str = entry
            .data
            .as_ref()
            .map(|d| format!(" | data: {}", d))
            .unwrap_or_default();

        writeln!(
            writer,
            "[{}] Frame {} | {} | {}{}",
            timestamp, entry.frame, category_str, entry.message, data_str
        )?;

        writer.flush()?;
        Ok(())
    }

    pub fn write_header(&self) -> std::io::Result<()> {
        let mut writer = self.writer.lock().unwrap();

        writeln!(writer, "=== CLAUDETEST3 DEBUG LOG ===")?;
        writeln!(writer, "Started at: {:?}", std::time::SystemTime::now())?;
        writeln!(
            writer,
            "Log format: [timestamp_ms] Frame # | CATEGORY | message | data"
        )?;
        writeln!(writer, "Session directory: {:?}", self.log_dir)?;
        writeln!(writer, "============================================")?;
        writeln!(writer)?;

        writer.flush()?;
        Ok(())
    }

    pub fn get_screenshot_path(&self, timestamp: u128) -> PathBuf {
        self.log_dir.join(format!("screenshot_{}.png", timestamp))
    }
}

impl Default for LogWriter {
    fn default() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let session_name = format!("session_{}", timestamp);

        Self::new(&session_name).expect("Failed to create log writer")
    }
}