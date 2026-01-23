use log::Level;

pub fn init() {
    let log_level = match option_env!("RUST_LOG") {
        Some("trace") => Level::Trace,
        Some("debug") => Level::Debug,
        Some("info") => Level::Info,
        Some("warn") => Level::Warn,
        Some("error") => Level::Error,
        _ => {
            if cfg!(debug_assertions) {
                Level::Debug
            } else {
                Level::Info
            }
        }
    };

    console_log::init_with_level(log_level).expect("Failed to initialize logging");
    log::info!("[INFO] [{}] Logging initialized with level: {}", module_path!().split("::").last().unwrap_or("unknown"), log_level);
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::trace!("[TRACE] [{}] {}", module_path!().split("::").last().unwrap_or("unknown"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!("[DEBUG] [{}] {}", module_path!().split("::").last().unwrap_or("unknown"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!("[INFO] [{}] {}", module_path!().split("::").last().unwrap_or("unknown"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!("[WARN] [{}] {}", module_path!().split("::").last().unwrap_or("unknown"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!("[ERROR] [{}] {}", module_path!().split("::").last().unwrap_or("unknown"), format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_does_not_panic() {
        init();
    }
}
