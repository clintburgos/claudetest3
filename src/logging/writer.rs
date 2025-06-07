use bevy::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::logging::components::*;

#[derive(Resource)]
pub struct LogWriter {
    writer: Arc<Mutex<BufWriter<File>>>,
    _log_path: PathBuf,
}

impl LogWriter {
    pub fn new(filename: &str) -> std::io::Result<Self> {
        let log_path = PathBuf::from(format!("logs/{}", filename));

        // Create logs directory if it doesn't exist
        std::fs::create_dir_all("logs")?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)?;

        let writer = Arc::new(Mutex::new(BufWriter::new(file)));

        Ok(Self {
            writer,
            _log_path: log_path,
        })
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
        writeln!(writer, "============================================")?;
        writeln!(writer)?;

        writer.flush()?;
        Ok(())
    }
}

impl Default for LogWriter {
    fn default() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = format!("debug_log_{}.txt", timestamp);

        Self::new(&filename).expect("Failed to create log writer")
    }
}
