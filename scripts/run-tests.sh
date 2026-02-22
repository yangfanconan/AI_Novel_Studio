#!/bin/bash

set -e

echo "=========================================="
echo "AI Novel Studio - Enterprise Test Suite"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

function run_backend_tests() {
    echo "📦 Running Backend Tests (Rust)..."
    echo "----------------------------------------"
    
    cd src-tauri
    
    echo "Running unit tests..."
    cargo test --lib -- --nocapture 2>&1 | tee test-output.txt
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo "✅ Backend tests passed"
    else
        echo "❌ Backend tests failed"
        return 1
    fi
    
    cd ..
    echo ""
}

function run_frontend_tests() {
    echo "🎨 Running Frontend Tests (TypeScript/React)..."
    echo "----------------------------------------"
    
    echo "Running unit tests with coverage..."
    npm run test:coverage 2>&1 | tee test-output.txt
    
    if [ $? -eq 0 ]; then
        echo "✅ Frontend tests passed"
    else
        echo "❌ Frontend tests failed"
        return 1
    fi
    
    echo ""
}

function run_integration_tests() {
    echo "🔗 Running Integration Tests..."
    echo "----------------------------------------"
    
    cd src-tauri
    
    echo "Running integration tests..."
    cargo test --test integration_tests -- --nocapture 2>&1 | tee test-output-integration.txt
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo "✅ Integration tests passed"
    else
        echo "❌ Integration tests failed"
        return 1
    fi
    
    cd ..
    echo ""
}

function run_e2e_tests() {
    echo "🎭 Running End-to-End Tests..."
    echo "----------------------------------------"
    
    echo "Running E2E tests..."
    npm run test:e2e 2>&1 | tee test-output-e2e.txt
    
    if [ $? -eq 0 ]; then
        echo "✅ E2E tests passed"
    else
        echo "❌ E2E tests failed"
        return 1
    fi
    
    echo ""
}

function check_code_coverage() {
    echo "📊 Checking Code Coverage..."
    echo "----------------------------------------"
    
    if [ -f "coverage/coverage-summary.json" ]; then
        echo "Coverage report generated"
        echo "Open coverage/index.html for detailed report"
    else
        echo "⚠️  Coverage report not found"
    fi
    
    echo ""
}

function run_linting() {
    echo "🔍 Running Code Linting..."
    echo "----------------------------------------"
    
    echo "Checking Rust code..."
    cd src-tauri
    cargo clippy -- -D warnings 2>&1 | tee test-output-clippy.txt || true
    
    echo "Checking TypeScript code..."
    cd ..
    npm run lint 2>&1 | tee test-output-lint.txt || true
    
    echo ""
}

function generate_test_report() {
    echo "📄 Generating Test Report..."
    echo "----------------------------------------"
    
    REPORT_FILE="test-report-$(date +%Y%m%d-%H%M%S).txt"
    
    cat > "$REPORT_FILE" << EOF
AI Novel Studio Test Report
Generated: $(date)

Backend Tests:
EOF
    
    if [ -f "src-tauri/test-output.txt" ]; then
        cat src-tauri/test-output.txt >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" << EOF

Frontend Tests:
EOF
    
    if [ -f "test-output.txt" ]; then
        cat test-output.txt >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" << EOF

Integration Tests:
EOF
    
    if [ -f "src-tauri/test-output-integration.txt" ]; then
        cat src-tauri/test-output-integration.txt >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" << EOF

E2E Tests:
EOF
    
    if [ -f "test-output-e2e.txt" ]; then
        cat test-output-e2e.txt >> "$REPORT_FILE"
    fi
    
    echo "✅ Test report generated: $REPORT_FILE"
    echo ""
}

function cleanup() {
    echo "🧹 Cleaning up test artifacts..."
    echo "----------------------------------------"
    
    find . -name "test-output*.txt" -mtime +7 -delete
    find . -name "test-report-*.txt" -mtime +30 -delete
    
    echo "✅ Cleanup completed"
    echo ""
}

MAIN_MENU="Select test suite to run:
1) Run Backend Tests (Rust)
2) Run Frontend Tests (React)
3) Run Integration Tests
4) Run E2E Tests
5) Run All Tests
6) Check Code Coverage
7) Run Linting
8) Generate Test Report
9) Cleanup Test Artifacts
10) Run Full CI Pipeline (All tests + linting)
0) Exit"

show_menu() {
    clear
    echo "=========================================="
    echo "AI Novel Studio - Test Runner"
    echo "=========================================="
    echo ""
    echo "$MAIN_MENU"
    echo ""
}

run_full_ci() {
    echo "🚀 Running Full CI Pipeline..."
    echo "=========================================="
    echo ""
    
    local exit_code=0
    
    run_linting || exit_code=1
    run_backend_tests || exit_code=1
    run_frontend_tests || exit_code=1
    run_integration_tests || exit_code=1
    run_e2e_tests || exit_code=1
    check_code_coverage
    generate_test_report
    
    echo ""
    echo "=========================================="
    if [ $exit_code -eq 0 ]; then
        echo "✅ Full CI Pipeline completed successfully!"
    else
        echo "❌ Full CI Pipeline failed with exit code: $exit_code"
    fi
    echo "=========================================="
    
    return $exit_code
}

show_menu

read -p "Enter your choice [0-10]: " choice

case $choice in
    1)
        run_backend_tests
        ;;
    2)
        run_frontend_tests
        ;;
    3)
        run_integration_tests
        ;;
    4)
        run_e2e_tests
        ;;
    5)
        run_backend_tests
        run_frontend_tests
        run_integration_tests
        run_e2e_tests
        echo "✅ All tests completed!"
        ;;
    6)
        check_code_coverage
        ;;
    7)
        run_linting
        ;;
    8)
        generate_test_report
        ;;
    9)
        cleanup
        ;;
    10)
        run_full_ci
        ;;
    0)
        echo "Exiting..."
        exit 0
        ;;
    *)
        echo "Invalid choice. Please try again."
        exit 1
        ;;
esac
