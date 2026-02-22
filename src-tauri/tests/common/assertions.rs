use std::result;

pub struct TestResult<T> {
    pub name: String,
    pub passed: bool,
    pub details: Option<String>,
    pub value: Option<T>,
}

impl<T> TestResult<T> {
    pub fn passed(name: &str, value: T) -> Self {
        TestResult {
            name: name.to_string(),
            passed: true,
            details: None,
            value: Some(value),
        }
    }

    pub fn failed(name: &str, details: &str) -> Self {
        TestResult {
            name: name.to_string(),
            passed: false,
            details: Some(details.to_string()),
            value: None,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.passed
    }

    pub fn unwrap(self) -> T {
        self.value.expect("Test failed, no value available")
    }
}

pub fn assert_eq<T: std::fmt::Debug + PartialEq>(name: &str, left: T, right: T) -> TestResult<()> {
    if left == right {
        TestResult::passed(name, ())
    } else {
        TestResult::failed(name, &format!("Expected {:?}, got {:?}", right, left))
    }
}

pub fn assert_ne<T: std::fmt::Debug + PartialEq>(name: &str, left: T, right: T) -> TestResult<()> {
    if left != right {
        TestResult::passed(name, ())
    } else {
        TestResult::failed(name, &format!("Expected not equal, but both were {:?}", left))
    }
}

pub fn assert_true(name: &str, value: bool) -> TestResult<()> {
    if value {
        TestResult::passed(name, ())
    } else {
        TestResult::failed(name, "Expected true, got false")
    }
}

pub fn assert_false(name: &str, value: bool) -> TestResult<()> {
    if !value {
        TestResult::passed(name, ())
    } else {
        TestResult::failed(name, "Expected false, got true")
    }
}

pub fn assert_some<T: std::fmt::Debug>(name: &str, value: Option<T>) -> TestResult<T> {
    match value {
        Some(v) => TestResult::passed(name, v),
        None => TestResult::failed(name, "Expected Some, got None"),
    }
}

pub fn assert_none<T: std::fmt::Debug>(name: &str, value: Option<T>) -> TestResult<()> {
    match value {
        None => TestResult::passed(name, ()),
        Some(v) => TestResult::failed(name, &format!("Expected None, got Some({:?})", v)),
    }
}

pub fn assert_contains(name: &str, haystack: &str, needle: &str) -> TestResult<()> {
    if haystack.contains(needle) {
        TestResult::passed(name, ())
    } else {
        TestResult::failed(name, &format!("Expected '{}' to contain '{}'", haystack, needle))
    }
}

pub fn assert_err<T, E: std::fmt::Debug>(name: &str, result: result::Result<T, E>) -> TestResult<E> {
    match result {
        Err(e) => TestResult::passed(name, e),
        Ok(_) => TestResult::failed(name, "Expected error, got Ok"),
    }
}

pub fn assert_ok<T, E: std::fmt::Debug>(name: &str, result: result::Result<T, E>) -> TestResult<T> {
    match result {
        Ok(v) => TestResult::passed(name, v),
        Err(e) => TestResult::failed(name, &format!("Expected Ok, got Err({:?})", e)),
    }
}

pub struct TestSuite {
    pub name: String,
    pub results: Vec<TestResult<()>>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        TestSuite {
            name: name.to_string(),
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult<()>) {
        self.results.push(result);
    }

    pub fn add_test<F>(&mut self, name: &str, test: F)
    where
        F: FnOnce() -> TestResult<()>,
    {
        self.add_result(test());
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub fn total_count(&self) -> usize {
        self.results.len()
    }

    pub fn is_all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    pub fn print_summary(&self) {
        println!("\n=== Test Suite: {} ===", self.name);
        println!("Total tests: {}", self.total_count());
        println!("Passed: {}", self.passed_count());
        println!("Failed: {}", self.failed_count());
        println!("Success rate: {:.1}%", 
            (self.passed_count() as f64 / self.total_count() as f64) * 100.0);

        if !self.is_all_passed() {
            println!("\nFailed tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  ✗ {}: {}", result.name, 
                        result.details.as_ref().unwrap_or(&"Unknown error".to_string()));
                }
            }
        }

        if self.is_all_passed() {
            println!("\n✓ All tests passed!");
        } else {
            println!("\n✗ Some tests failed!");
        }
    }
}
