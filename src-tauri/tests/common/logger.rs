use std::sync::{Arc, Mutex};
use std::io::Write;

pub struct TestLogger {
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl TestLogger {
    pub fn new() -> Self {
        TestLogger {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn capture(&self) -> String {
        let logs = self.logs.lock().unwrap();
        logs.join("\n")
    }

    pub fn clear(&self) {
        let mut logs = self.logs.lock().unwrap();
        logs.clear();
    }

    pub fn count(&self) -> usize {
        let logs = self.logs.lock().unwrap();
        logs.len()
    }

    pub fn contains(&self, pattern: &str) -> bool {
        let logs = self.logs.lock().unwrap();
        logs.iter().any(|log| log.contains(pattern))
    }

    pub fn assert_contains(&self, pattern: &str) {
        assert!(
            self.contains(pattern),
            "Expected logs to contain '{}', but didn't find it in:\n{}",
            pattern,
            self.capture()
        );
    }

    pub fn assert_not_contains(&self, pattern: &str) {
        assert!(
            !self.contains(pattern),
            "Expected logs NOT to contain '{}', but found it in:\n{}",
            pattern,
            self.capture()
        );
    }

    pub fn assert_count(&self, expected: usize) {
        let actual = self.count();
        assert_eq!(
            actual, expected,
            "Expected {} log entries, but got {}",
            expected,
            actual
        );
    }
}

impl Write for TestLogger {
    fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        let log = String::from_utf8_lossy(buf).to_string();
        let mut logs = self.logs.lock().unwrap();
        logs.push(log);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_capture() {
        let logger = TestLogger::new();
        writeln!(logger, "Test message").unwrap();
        
        assert!(logger.contains("Test message"));
    }

    #[test]
    fn test_logger_count() {
        let logger = TestLogger::new();
        writeln!(logger, "Message 1").unwrap();
        writeln!(logger, "Message 2").unwrap();
        writeln!(logger, "Message 3").unwrap();
        
        logger.assert_count(3);
    }

    #[test]
    fn test_logger_clear() {
        let logger = TestLogger::new();
        writeln!(logger, "Message").unwrap();
        
        assert_eq!(logger.count(), 1);
        
        logger.clear();
        
        assert_eq!(logger.count(), 0);
    }
}
