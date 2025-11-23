# Testing Guide for MCP Server: SearXNG

This document provides comprehensive testing procedures for the mcp-server-searxng Zed extension.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Manual Testing](#manual-testing)
- [Validation Tests](#validation-tests)
- [Integration Tests](#integration-tests)
- [Error Handling Tests](#error-handling-tests)
- [Performance Testing](#performance-testing)
- [Regression Testing](#regression-testing)

## Prerequisites

### Required Tools

- **Zed Editor**: Version 0.205.x or higher
- **Rust**: Latest stable with `wasm32-wasip2` target
- **Node.js**: Version 20 or higher
- **SearXNG Instance**: Access to public or self-hosted instance

### Installation Check

```bash
# Verify Rust and target
rustup --version
rustup target list | grep wasm32-wasip2

# Verify Node.js and npx
node --version
npx --version

# Test SearXNG instance (replace URL)
curl "https://searx.be/search?q=test&format=json"
```

## Development Setup

### 1. Build the Extension

```bash
# Clean build
cargo clean

# Check for errors
cargo check --target wasm32-wasip2

# Run clippy
cargo clippy --target wasm32-wasip2 -- -D warnings

# Format code
cargo fmt

# Build release
cargo build --target wasm32-wasip2 --release

# Verify binary size (should be < 500KB)
ls -lh target/wasm32-wasip2/release/*.wasm
```

**Expected Output:**
- No compilation errors
- No clippy warnings
- Binary size: ~150KB
- Clean formatting

### 2. Install as Dev Extension

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Search for "zed: install dev extension"
4. Select the `mcp-server-searxng` directory
5. Wait for confirmation message

**Expected:** "Extension installed successfully" message

### 3. View Extension Logs

```bash
# In Zed Command Palette
zed: open log

# Or run Zed in foreground mode for verbose logging
zed --foreground
```

## Manual Testing

### Test 1: Basic Configuration

**Objective:** Verify extension loads with minimal configuration

**Steps:**
1. Open Zed settings: `Cmd+,` or Settings > Open Settings
2. Add configuration:
```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://searx.be"
      }
    }
  }
}
```
3. Save settings
4. Restart Zed or reload extension
5. Open AI Assistant panel
6. Check extension logs

**Expected Results:**
- ✅ No errors in logs
- ✅ Extension shows as loaded
- ✅ AI Assistant panel accessible

**Pass Criteria:** Extension loads without errors

---

### Test 2: Search Functionality

**Objective:** Verify web search tool is available and functional

**Steps:**
1. Open AI Assistant in Zed
2. Start a conversation
3. Ask: "Search for Rust async programming best practices"
4. Observe tool invocation and results

**Expected Results:**
- ✅ AI recognizes search intent
- ✅ `searxng_web_search` tool is invoked
- ✅ Search results are returned
- ✅ AI incorporates results into response
- ✅ Results include titles, URLs, and snippets

**Pass Criteria:** Search completes within 10 seconds with relevant results

---

### Test 3: URL Reading

**Objective:** Verify URL content reading tool works

**Steps:**
1. Ask AI: "Read this URL and summarize: https://docs.searxng.org/"
2. Observe tool invocation

**Expected Results:**
- ✅ `web_url_read` tool is invoked
- ✅ Page content is fetched
- ✅ Content is converted to markdown
- ✅ AI provides summary based on content

**Pass Criteria:** URL reading completes within 15 seconds

---

## Validation Tests

### Test 4: URL Validation - Valid URLs

**Objective:** Verify valid URLs are accepted

**Test Cases:**

| URL | Expected Result |
|-----|-----------------|
| `https://searx.be` | ✅ Pass |
| `http://localhost:8888` | ✅ Pass |
| `https://search.example.com` | ✅ Pass |
| `https://searx.example.com:443` | ✅ Pass |

**Steps:**
1. For each URL, update settings
2. Reload extension
3. Check logs for errors

**Pass Criteria:** All valid URLs load without errors

---

### Test 5: URL Validation - Invalid URLs

**Objective:** Verify invalid URLs are rejected with helpful errors

**Test Cases:**

| URL | Expected Error |
|-----|----------------|
| `searx.be` (no protocol) | "URL must start with 'http://' or 'https://'" |
| `https://searx.be/` (trailing slash) | "URL should not end with a trailing slash" |
| `` (empty) | "SEARXNG_URL cannot be empty" |

**Steps:**
1. For each invalid URL, update settings
2. Reload extension
3. Check error message in logs
4. Verify error message is helpful

**Pass Criteria:** Clear, actionable error messages for all invalid inputs

---

### Test 6: Missing Configuration

**Objective:** Verify helpful error when SEARXNG_URL is missing

**Steps:**
1. Remove `searxng_url` from settings or use empty object:
```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {}
    }
  }
}
```
2. Reload extension
3. Check error message

**Expected Error Message:**
```
SEARXNG_URL environment variable not set.

Please add this to your Zed settings.json:

{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://searx.be"
      }
    }
  }
}

Find public instances at: https://searx.space/
```

**Pass Criteria:** Error message includes:
- ✅ Clear problem statement
- ✅ Exact configuration example
- ✅ Link to find instances

---

## Integration Tests

### Test 7: Authenticated SearXNG Instance

**Objective:** Verify authentication credentials are passed correctly

**Prerequisites:** Access to password-protected SearXNG instance

**Steps:**
1. Configure with auth:
```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://your-instance.com",
        "auth_username": "testuser",
        "auth_password": "testpass"
      }
    }
  }
}
```
2. Perform search
3. Check logs for auth headers

**Expected Results:**
- ✅ Search succeeds with auth
- ✅ No credential leakage in logs
- ✅ Proper Basic Auth header sent

**Pass Criteria:** Authentication works without exposing credentials in logs

---

### Test 8: Proxy Configuration

**Objective:** Verify proxy settings are respected

**Prerequisites:** Access to HTTP proxy

**Steps:**
1. Configure with proxy:
```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://searx.be",
        "http_proxy": "http://proxy.example.com:8080",
        "https_proxy": "http://proxy.example.com:8080"
      }
    }
  }
}
```
2. Perform search
3. Verify traffic goes through proxy

**Expected Results:**
- ✅ Requests route through proxy
- ✅ Search completes successfully

**Pass Criteria:** Proxy configuration is honored

---

### Test 9: Public SearXNG Instances

**Objective:** Test with multiple public instances

**Test Instances:**
- https://searx.be
- https://search.disroot.org
- https://searx.tiekoetter.com

**Steps:**
1. For each instance:
   - Update `searxng_url`
   - Perform test search
   - Record response time
   - Verify result quality

**Expected Results:**
- ✅ Works with all major public instances
- ✅ Response time < 10 seconds
- ✅ Results are relevant

**Pass Criteria:** Extension works with at least 3 public instances

---

## Error Handling Tests

### Test 10: SearXNG Instance Down

**Objective:** Verify graceful handling of unreachable instance

**Steps:**
1. Configure with non-existent URL:
```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://does-not-exist-xyz123.com"
      }
    }
  }
}
```
2. Attempt search
3. Check error message

**Expected Results:**
- ✅ Extension doesn't crash
- ✅ Helpful error message
- ✅ Suggests checking URL and network

**Pass Criteria:** Graceful degradation with actionable error

---

### Test 11: Network Timeout

**Objective:** Verify timeout handling

**Steps:**
1. Use a very slow SearXNG instance or simulate timeout
2. Attempt search
3. Wait for timeout (should be ~30 seconds)

**Expected Results:**
- ✅ Request times out gracefully
- ✅ Error message indicates timeout
- ✅ Extension remains functional

**Pass Criteria:** Timeout doesn't crash extension

---

### Test 12: Invalid JSON Response

**Objective:** Verify handling of malformed SearXNG responses

**Prerequisites:** This is handled by mcp-searxng npm package

**Steps:**
1. Monitor logs during various searches
2. Look for parsing errors
3. Verify extension continues to function

**Expected Results:**
- ✅ Malformed responses logged but don't crash extension
- ✅ Subsequent searches still work

**Pass Criteria:** Resilience to bad data

---

## Performance Testing

### Test 13: Search Response Time

**Objective:** Measure search performance

**Steps:**
1. Perform 10 test searches
2. Record time from request to response
3. Calculate average, min, max

**Measurements:**
| Search Query | Time (seconds) |
|--------------|----------------|
| "Rust async" | ___ |
| "Python testing" | ___ |
| "JavaScript frameworks" | ___ |
| "Docker containers" | ___ |
| "Git workflow" | ___ |
| "REST API design" | ___ |
| "Linux commands" | ___ |
| "React hooks" | ___ |
| "SQL optimization" | ___ |
| "Machine learning" | ___ |

**Expected Results:**
- ✅ Average < 5 seconds
- ✅ Max < 10 seconds
- ✅ No degradation over time

**Pass Criteria:** Performance within acceptable bounds

---

### Test 14: Memory Usage

**Objective:** Verify extension doesn't leak memory

**Steps:**
1. Open Zed with extension loaded
2. Note initial memory usage (Activity Monitor / Task Manager)
3. Perform 20+ searches
4. Note memory usage after searches
5. Wait 5 minutes idle
6. Check memory usage again

**Expected Results:**
- ✅ WASM binary < 50MB in memory
- ✅ No memory growth after searches
- ✅ Memory released when idle

**Pass Criteria:** Memory usage remains stable

---

### Test 15: Concurrent Requests

**Objective:** Verify handling of multiple simultaneous searches

**Steps:**
1. Open multiple AI Assistant conversations
2. Send search queries to all simultaneously
3. Monitor logs and responses

**Expected Results:**
- ✅ All requests complete successfully
- ✅ No race conditions
- ✅ Proper queuing or parallel handling

**Pass Criteria:** Concurrent requests handled gracefully

---

## Regression Testing

### Test 16: Configuration Changes

**Objective:** Verify settings updates are applied

**Steps:**
1. Start with configuration A
2. Perform search - note instance used
3. Change to configuration B (different instance)
4. Reload extension or restart AI Assistant
5. Perform search - verify new instance used

**Expected Results:**
- ✅ Changes are detected
- ✅ New configuration applied
- ✅ No restart of Zed required (just AI Assistant)

**Pass Criteria:** Configuration updates work as expected

---

### Test 17: Extension Reload

**Objective:** Verify extension handles reload gracefully

**Steps:**
1. Load extension
2. Perform several searches
3. Reload extension via dev tools
4. Perform searches again

**Expected Results:**
- ✅ Extension reloads without errors
- ✅ State is properly reset
- ✅ Functionality restored

**Pass Criteria:** Clean reload behavior

---

## Success Criteria Summary

**Phase 2 is complete when:**

- [ ] Extension successfully launches mcp-searxng via npx
- [ ] Configuration from settings.json is applied correctly
- [ ] SEARXNG_URL validation works with clear error messages
- [ ] All environment variables (auth, proxy, etc.) are passed
- [ ] Manual testing with test SearXNG instance works
- [ ] Error messages are helpful and actionable
- [ ] No clippy warnings
- [ ] Performance within specified bounds (< 10s for searches)
- [ ] All critical test cases pass

## Test Report Template

```markdown
# Test Report: mcp-server-searxng v0.1.0

**Date:** YYYY-MM-DD
**Tester:** Your Name
**Zed Version:** x.x.x
**Node.js Version:** x.x.x
**OS:** macOS/Linux/Windows

## Test Results

| Test # | Test Name | Status | Notes |
|--------|-----------|--------|-------|
| 1 | Basic Configuration | ✅/❌ | |
| 2 | Search Functionality | ✅/❌ | |
| 3 | URL Reading | ✅/❌ | |
| 4 | URL Validation - Valid | ✅/❌ | |
| 5 | URL Validation - Invalid | ✅/❌ | |
| 6 | Missing Configuration | ✅/❌ | |
| 7 | Authenticated Instance | ✅/❌ | |
| 8 | Proxy Configuration | ✅/❌ | |
| 9 | Public Instances | ✅/❌ | |
| 10 | Instance Down | ✅/❌ | |
| 11 | Network Timeout | ✅/❌ | |
| 12 | Invalid JSON | ✅/❌ | |
| 13 | Search Response Time | ✅/❌ | Avg: ___s |
| 14 | Memory Usage | ✅/❌ | Max: ___MB |
| 15 | Concurrent Requests | ✅/❌ | |
| 16 | Configuration Changes | ✅/❌ | |
| 17 | Extension Reload | ✅/❌ | |

## Summary

**Total Tests:** 17
**Passed:** ___
**Failed:** ___
**Pass Rate:** ___%

## Critical Issues

(List any critical bugs found)

## Non-Critical Issues

(List any minor issues or improvements)

## Recommendations

(Next steps or suggestions)
```

## Troubleshooting Test Issues

### Extension Won't Load

**Check:**
- Zed version compatibility
- WASM binary exists at `target/wasm32-wasip2/release/mcp_server_searxng.wasm`
- Extension directory path is correct
- Zed logs for specific error messages

### Searches Failing

**Check:**
- SearXNG instance is accessible (test with curl)
- JSON format enabled on instance
- Authentication credentials if required
- Network connectivity and proxy settings
- Zed logs for detailed error messages

### Tools Not Appearing

**Check:**
- AI Assistant is enabled in Zed
- Extension is properly loaded (check extensions panel)
- Context servers configuration is correct
- Restart AI Assistant or Zed

## Additional Resources

- **Zed Extension Development**: https://zed.dev/docs/extensions/developing-extensions
- **MCP Specification**: https://modelcontextprotocol.io/
- **SearXNG Documentation**: https://docs.searxng.org/
- **Extension Repository**: https://github.com/swannysec/mcp-server-searxng