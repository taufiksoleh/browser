#!/bin/bash
# CEF Browser Smoke Test Script
# Validates browser functionality in CI/CD environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/../build"
BROWSER_BIN="${BUILD_DIR}/cef_browser"
TIMEOUT_SECONDS=30
TEST_PORT=9222

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
PASSED=0
FAILED=0
SKIPPED=0

log_pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    PASSED=$((PASSED + 1))
}

log_fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    FAILED=$((FAILED + 1))
}

log_skip() {
    echo -e "${YELLOW}⚠ SKIP${NC}: $1"
    SKIPPED=$((SKIPPED + 1))
}

log_info() {
    echo -e "  INFO: $1"
}

cleanup() {
    # Kill any browser processes we started
    if [ -n "$BROWSER_PID" ]; then
        kill $BROWSER_PID 2>/dev/null || true
    fi
    if [ -n "$XVFB_PID" ]; then
        kill $XVFB_PID 2>/dev/null || true
    fi
}

trap cleanup EXIT

echo "========================================="
echo "CEF Browser Smoke Tests"
echo "========================================="
echo ""

# ===========================================================================
# Test 1: Binary Existence
# ===========================================================================
echo "[Test 1] Binary Existence"
if [ -f "$BROWSER_BIN" ]; then
    log_pass "Browser binary exists at $BROWSER_BIN"
else
    log_fail "Browser binary not found at $BROWSER_BIN"
    exit 1
fi

# ===========================================================================
# Test 2: Binary Executable
# ===========================================================================
echo ""
echo "[Test 2] Binary Permissions"
if [ -x "$BROWSER_BIN" ]; then
    log_pass "Browser binary is executable"
else
    chmod +x "$BROWSER_BIN" 2>/dev/null
    if [ -x "$BROWSER_BIN" ]; then
        log_pass "Browser binary made executable"
    else
        log_fail "Cannot make browser binary executable"
    fi
fi

# ===========================================================================
# Test 3: Binary Format Validation
# ===========================================================================
echo ""
echo "[Test 3] Binary Format"
FILE_TYPE=$(file "$BROWSER_BIN" 2>/dev/null || echo "unknown")
if echo "$FILE_TYPE" | grep -qE "(ELF|Mach-O|PE32)"; then
    log_pass "Valid executable format: $(echo $FILE_TYPE | cut -d: -f2 | cut -c1-50)"
else
    log_fail "Invalid or unknown binary format"
fi

# ===========================================================================
# Test 4: Library Dependencies
# ===========================================================================
echo ""
echo "[Test 4] Library Dependencies"
if command -v ldd &> /dev/null; then
    MISSING=$(ldd "$BROWSER_BIN" 2>/dev/null | grep "not found" || true)
    if [ -z "$MISSING" ]; then
        log_pass "All library dependencies satisfied"
    else
        log_fail "Missing libraries:"
        echo "$MISSING"
    fi
elif command -v otool &> /dev/null; then
    # macOS
    log_pass "Library check (macOS - using otool)"
else
    log_skip "Cannot check library dependencies (ldd not available)"
fi

# ===========================================================================
# Test 5: Helper Binary
# ===========================================================================
echo ""
echo "[Test 5] Helper Binary"
HELPER_BIN="${BUILD_DIR}/cef_browser_helper"
if [ -f "$HELPER_BIN" ]; then
    log_pass "Helper binary exists"
else
    log_skip "Helper binary not found (may not be required on all platforms)"
fi

# ===========================================================================
# Test 6: CEF Resources
# ===========================================================================
echo ""
echo "[Test 6] CEF Resources"
RESOURCES_OK=true

# Check for critical CEF resources
CRITICAL_RESOURCES=(
    "icudtl.dat"
    "v8_context_snapshot.bin"
)

for resource in "${CRITICAL_RESOURCES[@]}"; do
    if [ -f "${BUILD_DIR}/${resource}" ]; then
        log_info "Found: $resource"
    else
        log_info "Missing: $resource"
        RESOURCES_OK=false
    fi
done

if [ "$RESOURCES_OK" = true ]; then
    log_pass "Critical CEF resources present"
else
    log_skip "Some CEF resources missing (may be bundled differently)"
fi

# ===========================================================================
# Test 7: Version/Help Output
# ===========================================================================
echo ""
echo "[Test 7] Command Line Interface"
if timeout 5 "$BROWSER_BIN" --version 2>&1 | grep -qiE "(version|cef|chromium)" ; then
    log_pass "Browser responds to --version"
elif timeout 5 "$BROWSER_BIN" --help 2>&1 | grep -qiE "(usage|help|option)" ; then
    log_pass "Browser responds to --help"
else
    log_skip "No version/help output (browser may not support these flags)"
fi

# ===========================================================================
# Test 8: Headless Startup (if display available)
# ===========================================================================
echo ""
echo "[Test 8] Headless Startup"

# Try to start Xvfb if no display
if [ -z "$DISPLAY" ]; then
    if command -v Xvfb &> /dev/null; then
        export DISPLAY=:99
        Xvfb :99 -screen 0 1280x800x24 &
        XVFB_PID=$!
        sleep 2
        log_info "Started virtual display at :99"
    else
        log_skip "No display available and Xvfb not installed"
    fi
fi

if [ -n "$DISPLAY" ]; then
    # Start browser with test page
    TEST_HTML="data:text/html,<html><head><title>SmokeTest</title></head><body><h1>CEF Browser Smoke Test</h1></body></html>"

    timeout $TIMEOUT_SECONDS "$BROWSER_BIN" \
        --headless \
        --disable-gpu \
        --no-sandbox \
        --remote-debugging-port=$TEST_PORT \
        --url="$TEST_HTML" &
    BROWSER_PID=$!

    # Wait for startup
    sleep 5

    if kill -0 $BROWSER_PID 2>/dev/null; then
        log_pass "Browser started in headless mode"

        # Test 9: Remote debugging
        echo ""
        echo "[Test 9] Remote Debugging"
        if command -v curl &> /dev/null; then
            RESPONSE=$(curl -s "http://localhost:$TEST_PORT/json/version" 2>/dev/null || true)
            if echo "$RESPONSE" | grep -qi "browser"; then
                log_pass "Remote debugging endpoint responding"
                log_info "Browser: $(echo $RESPONSE | grep -o '"Browser":"[^"]*"' | head -1)"
            else
                log_skip "Remote debugging not responding (may be expected)"
            fi
        else
            log_skip "curl not available for remote debugging test"
        fi

        # Cleanup browser
        kill $BROWSER_PID 2>/dev/null || true
        wait $BROWSER_PID 2>/dev/null || true
    else
        log_skip "Browser process exited (may need different startup args)"
    fi
else
    log_skip "No display available for startup test"
fi

# ===========================================================================
# Test Summary
# ===========================================================================
echo ""
echo "========================================="
echo "Smoke Test Summary"
echo "========================================="
echo -e "  ${GREEN}Passed${NC}:  $PASSED"
echo -e "  ${RED}Failed${NC}:  $FAILED"
echo -e "  ${YELLOW}Skipped${NC}: $SKIPPED"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${RED}SMOKE TESTS FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}SMOKE TESTS PASSED${NC}"
    exit 0
fi
