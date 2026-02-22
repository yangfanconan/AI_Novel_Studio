use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct Logger {
    feature: Option<String>,
    action: Option<String>,
    request_id: Option<String>,
    parent_request_id: Option<String>,
    depth: usize,
    log_file: Arc<std::sync::Mutex<Option<File>>>,
    min_level: LogLevel,
}

static GLOBAL_REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        let request_id = generate_request_id();
        let log_file = Self::init_log_file();
        Logger {
            feature: None,
            action: None,
            request_id: Some(request_id),
            parent_request_id: None,
            depth: 0,
            log_file: Arc::new(std::sync::Mutex::new(log_file)),
            min_level: LogLevel::Info,
        }
    }

    fn init_log_file() -> Option<File> {
        if let Ok(app_dir) = std::env::var("APP_LOG_DIR") {
            let log_path = PathBuf::from(app_dir).join("novel_studio.log");
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
            {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let _ = writeln!(file, "\n=== New Session: {} ===", timestamp);
                return Some(file);
            }
        }
        None
    }

    pub fn set_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }

    pub fn with_feature(mut self, feature: &str) -> Self {
        self.feature = Some(feature.to_string());
        self
    }

    pub fn with_action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }

    pub fn with_parent_request_id(mut self, parent_id: &str) -> Self {
        self.parent_request_id = Some(parent_id.to_string());
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    fn format_message(&self, level: LogLevel, message: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let request_id = self.request_id.as_deref().unwrap_or("unknown");
        let feature = self.feature.as_deref().unwrap_or("unknown");
        let action = self.action.as_ref().map(|a| a.as_str()).unwrap_or("");
        let parent_id = self.parent_request_id.as_deref().unwrap_or("none");
        
        let indent = "  ".repeat(self.depth);
        
        if action.is_empty() {
            format!("[{}][{}][req:{}] [feat:{}] [parent:{}] {}{}", 
                timestamp, level, request_id, feature, parent_id, indent, message)
        } else {
            format!("[{}][{}][req:{}] [feat:{}] [action:{}] [parent:{}] {}{}", 
                timestamp, level, request_id, feature, action, parent_id, indent, message)
        }
    }

    fn write_to_file(&self, formatted: &str) {
        if let Ok(mut guard) = self.log_file.lock() {
            if let Some(ref mut file) = *guard {
                let _ = writeln!(file, "{}", formatted);
                let _ = file.flush();
            }
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if level < self.min_level {
            return;
        }

        let formatted = self.format_message(level, message);
        self.write_to_file(&formatted);

        match level {
            LogLevel::Debug => println!("{}", formatted),
            LogLevel::Info => println!("{}", formatted),
            LogLevel::Warn => eprintln!("{}", formatted),
            LogLevel::Error => eprintln!("{}", formatted),
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn info_with_data(&self, message: &str, data: serde_json::Value) {
        let formatted = format!("{} | Data: {}", message, data);
        self.log(LogLevel::Info, &formatted);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn error_with_cause(&self, message: &str, cause: &dyn std::error::Error) {
        self.error(&format!("{} | Cause: {} | Type: {}", 
            message, 
            cause, 
            std::any::type_name_of_val(cause)
        ));
    }

    pub fn error_with_stack(&self, message: &str) {
        self.error(message);
        let stack = std::backtrace::Backtrace::capture();
        self.error(&format!("Stack trace:\n{}", stack));
    }

    pub fn track_action<F, R>(&self, action_name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let action_logger = self.clone().with_action(action_name);
        action_logger.info("Action started");
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        
        let duration = start.elapsed().as_millis();
        
        match result {
            Ok(r) => {
                action_logger.info(&format!("Action completed | Duration: {}ms", duration));
                r
            }
            Err(panic) => {
                action_logger.error(&format!("Action panicked after {}ms", duration));
                if let Some(msg) = panic.downcast_ref::<&str>() {
                    action_logger.error(&format!("Panic message: {}", msg));
                } else if let Some(msg) = panic.downcast_ref::<String>() {
                    action_logger.error(&format!("Panic message: {}", msg));
                }
                std::panic::resume_unwind(panic);
            }
        }
    }

    pub async fn track_async_action<F, R>(&self, action_name: &str, f: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        let start = std::time::Instant::now();
        let action_logger = self.clone().with_action(action_name);
        action_logger.info("Async action started");
        
        let result = f.await;
        
        let duration = start.elapsed().as_millis();
        action_logger.info(&format!("Async action completed | Duration: {}ms", duration));
        
        result
    }

    pub fn track_call_chain<F, R>(&self, feature: &str, action: &str, f: F) -> R
    where
        F: FnOnce(&Logger) -> R,
    {
        let call_logger = self.clone()
            .with_feature(feature)
            .with_action(action);
        
        call_logger.info("Call started");
        call_logger.debug(&format!("Call stack depth: {}", self.depth));
        
        let result = f(&call_logger);
        
        call_logger.info("Call completed successfully");
        result
    }
}

fn generate_request_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = GLOBAL_REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}-{:x}", timestamp, counter)
}

pub fn log_command_start(logger: &Logger, command_name: &str, params: &str) {
    let command_logger = logger.clone()
        .with_feature("tauri-command")
        .with_action(command_name);
    
    command_logger.info(&format!("Command started | Params: {}", params));
    command_logger.debug(&format!("Full parameters: {}", params));
}

pub fn log_command_success(logger: &Logger, command_name: &str, result: &str) {
    let command_logger = logger.clone()
        .with_feature("tauri-command")
        .with_action(command_name);
    
    command_logger.info(&format!("Command succeeded | Result: {}", result));
    command_logger.debug("Command execution completed successfully");
}

pub fn log_command_error(logger: &Logger, command_name: &str, error: &str) {
    let command_logger = logger.clone()
        .with_feature("tauri-command")
        .with_action(command_name);
    
    command_logger.error(&format!("Command failed | Error: {}", error));
    command_logger.error_with_stack(&format!("Error in command: {}", command_name));
}

pub fn log_database_operation(logger: &Logger, operation: &str, table: &str, details: &str) {
    let db_logger = logger.clone()
        .with_feature("database")
        .with_action(operation);
    
    db_logger.info(&format!("Database operation | Table: {} | Details: {}", table, details));
}

pub fn log_ai_operation(logger: &Logger, operation: &str, model: &str, details: &str) {
    let ai_logger = logger.clone()
        .with_feature("ai-service")
        .with_action(operation);
    
    ai_logger.info(&format!("AI operation | Model: {} | Details: {}", model, details));
}

pub fn log_validation_error(logger: &Logger, field: &str, reason: &str) {
    let validation_logger = logger.clone()
        .with_feature("validation")
        .with_action("validate");
    
    validation_logger.error(&format!("Validation failed | Field: {} | Reason: {}", field, reason));
}

pub fn log_performance_metric(logger: &Logger, metric_name: &str, value: f64, unit: &str) {
    let perf_logger = logger.clone()
        .with_feature("performance")
        .with_action("metric");
    
    perf_logger.info(&format!("Performance metric | {} = {} {}", metric_name, value, unit));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new();
        assert!(logger.request_id.is_some());
        assert!(logger.feature.is_none());
        assert!(logger.action.is_none());
    }

    #[test]
    fn test_logger_with_feature() {
        let logger = Logger::new().with_feature("test-feature");
        assert_eq!(logger.feature, Some("test-feature".to_string()));
    }

    #[test]
    fn test_logger_with_action() {
        let logger = Logger::new().with_action("test-action");
        assert_eq!(logger.action, Some("test-action".to_string()));
    }

    #[test]
    fn test_logger_with_min_level() {
        let logger = Logger::new().set_min_level(LogLevel::Warn);
        assert_eq!(logger.min_level, LogLevel::Warn);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Warn), "WARN");
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
    }

    #[test]
    fn test_request_id_generation() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert_ne!(id1, id2);
        assert!(id1.len() > 0);
    }

    #[test]
    fn test_log_level_filtering() {
        let logger = Logger::new().set_min_level(LogLevel::Warn);
        let logger_debug = logger.clone();
        let logger_warn = logger.clone();
        
        logger_debug.debug("This should not be logged");
        logger_warn.warn("This should be logged");
        
        assert_eq!(logger_debug.min_level, LogLevel::Warn);
        assert_eq!(logger_warn.min_level, LogLevel::Warn);
    }

    #[test]
    fn test_track_action() {
        let logger = Logger::new();
        let result = logger.track_action("test_action", || 42);
        assert_eq!(result, 42);
    }
}
