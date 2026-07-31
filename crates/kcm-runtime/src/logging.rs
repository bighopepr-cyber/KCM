use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

static LOG_LEVEL: AtomicU8 = AtomicU8::new(2);

pub fn set_log_level(level: LogLevel) {
    LOG_LEVEL.store(level as u8, Ordering::Relaxed);
}

pub fn get_log_level() -> LogLevel {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        0 => LogLevel::Error,
        1 => LogLevel::Warn,
        2 => LogLevel::Info,
        3 => LogLevel::Debug,
        4 => LogLevel::Trace,
        _ => LogLevel::Info,
    }
}

pub fn log(level: LogLevel, message: &str) {
    if level <= get_log_level() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        eprintln!("[{}] [{}] {}", timestamp, level, message);
    }
}

pub fn log_error(message: &str) {
    log(LogLevel::Error, message);
}
pub fn log_warn(message: &str) {
    log(LogLevel::Warn, message);
}
pub fn log_info(message: &str) {
    log(LogLevel::Info, message);
}
pub fn log_debug(message: &str) {
    log(LogLevel::Debug, message);
}
pub fn log_trace(message: &str) {
    log(LogLevel::Trace, message);
}
