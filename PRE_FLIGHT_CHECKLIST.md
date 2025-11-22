# Pre-Flight Checklist for mcp-server-searxng v0.1.0

This checklist must be completed before publishing the extension to the Zed marketplace.

## Date: ___________
## Reviewer: ___________
## Zed Version Tested: ___________
## Node.js Version Tested: ___________

---

## 1. Code Quality ✅

### Build & Compilation
- [ ] `cargo check --target wasm32-wasip2` passes with no errors
- [ ] `cargo build --target wasm32-wasip2 --release` succeeds
- [ ] WASM binary size is < 500KB (current: ~150KB)
- [ ] No compilation warnings

### Code Standards
- [ ] `cargo clippy --target wasm32-wasip2 -- -D warnings` passes with zero warnings
- [ ] `cargo fmt --check` shows all code is properly formatted
- [ ] All functions have documentation comments
- [ ] No `unwrap()` or `expect()` in production code paths
- [ ] Error messages are user-friendly and actionable

### Code Review
- [ ] All TODO comments resolved or tracked as issues
- [ ] No debug print statements or commented-out code
- [ ] Variable names are descriptive and follow Rust conventions
- [ ] Functions are focused and under 50 lines where possible
- [ ] No hardcoded secrets or credentials

**Notes:**
```
[Any issues found]
```

---

## 2. Documentation Completeness ✅

### README.md
- [ ] Installation instructions are clear and complete
- [ ] All configuration options documented
- [ ] Configuration examples cover common scenarios (basic, auth, proxy)
- [ ] Troubleshooting section addresses common issues
- [ ] Privacy & security section is accurate
- [ ] Links to resources are valid and working
- [ ] Screenshots or examples are up-to-date (if included)
- [ ] Contributing guidelines present

### TESTING.md
- [ ] All 17 test cases are documented
- [ ] Test procedures are clear and reproducible
- [ ] Success criteria defined for each test
- [ ] Test report template included

### CHANGELOG.md
- [ ] All changes documented
- [ ] Version numbers follow semantic versioning
- [ ] Release date set (or TBD for unreleased)
- [ ] Breaking changes clearly marked (if any)

### Code Documentation
- [ ] All public functions have doc comments
- [ ] Complex logic has inline comments
- [ ] Module-level documentation present

### Supporting Files
- [ ] LICENSE file present (MIT)
- [ ] extension.toml has complete metadata
- [ ] Cargo.toml has accurate description and license
- [ ] .gitignore excludes appropriate files

**Notes:**
```
[Any documentation gaps]
```

---

## 3. Functional Testing ✅

### Basic Functionality
- [ ] Extension installs successfully in Zed (dev extension)
- [ ] Extension appears in extensions list
- [ ] No errors on startup in Zed logs
- [ ] AI Assistant recognizes context server tools

### Configuration Tests
- [ ] Minimal configuration works (searxng_url only)
- [ ] Authenticated instance configuration works
- [ ] Proxy configuration works
- [ ] Custom user-agent configuration works
- [ ] Configuration changes apply after AI Assistant restart

### Search Tests
- [ ] `searxng_web_search` tool appears in AI Assistant
- [ ] Basic search query returns results
- [ ] Search results include title, URL, snippet
- [ ] Empty search results handled gracefully
- [ ] Search completes within 10 seconds

### URL Reading Tests
- [ ] `web_url_read` tool appears in AI Assistant
- [ ] Can fetch and convert web pages to markdown
- [ ] URL reading completes within 15 seconds
- [ ] Handles various page formats (articles, docs, blogs)

### Multiple Instance Tests
- [ ] Tested with at least 3 different public instances:
  - [ ] https://searx.be
  - [ ] https://search.disroot.org
  - [ ] https://searx.tiekoetter.com (or other)
- [ ] All instances return valid results

**Notes:**
```
[Test results and any failures]
```

---

## 4. Validation & Error Handling ✅

### URL Validation
- [ ] Rejects URLs without protocol (e.g., "searx.be")
- [ ] Rejects URLs with trailing slash
- [ ] Rejects empty URL with helpful error
- [ ] Accepts valid http:// URLs
- [ ] Accepts valid https:// URLs
- [ ] Error messages include fix suggestions

### Configuration Validation
- [ ] Missing SEARXNG_URL shows helpful error with example
- [ ] Error message includes link to find instances
- [ ] Invalid configuration doesn't crash extension
- [ ] Settings schema validates input correctly

### Network Error Handling
- [ ] Handles unreachable SearXNG instance gracefully
- [ ] Network timeout doesn't crash extension (30s timeout)
- [ ] Connection refused error is clear
- [ ] 401/403 authentication errors are clear
- [ ] 404 errors handled appropriately

### Edge Cases
- [ ] Handles malformed JSON responses
- [ ] Handles empty search results
- [ ] Handles very long URLs
- [ ] Handles special characters in search queries
- [ ] Handles concurrent requests

**Notes:**
```
[Any edge cases that need attention]
```

---

## 5. Performance Testing ✅

### Response Time
- [ ] Average search time < 5 seconds (tested with 10 queries)
- [ ] Maximum search time < 10 seconds
- [ ] URL reading time < 15 seconds for average pages
- [ ] Extension loads in < 2 seconds

### Resource Usage
- [ ] WASM memory usage < 50MB
- [ ] No memory leaks after multiple searches (tested 20+)
- [ ] Memory released when idle
- [ ] No CPU spikes during idle time

### Concurrent Operations
- [ ] Multiple simultaneous searches complete successfully
- [ ] No race conditions observed
- [ ] Proper request queuing or parallelization

**Performance Metrics:**
```
Average search time: _____s
Max search time: _____s
Average URL read time: _____s
Peak memory usage: _____MB
```

---

## 6. Security Review ✅

### Credentials & Secrets
- [ ] No API keys hardcoded in source
- [ ] No credentials in git history
- [ ] No credentials in logs or error messages
- [ ] Auth credentials marked as sensitive in docs
- [ ] Warning about credential storage in README

### Input Validation
- [ ] All user inputs sanitized
- [ ] URL format validation prevents injection
- [ ] No command injection vulnerabilities
- [ ] Environment variables validated

### Dependencies
- [ ] All dependencies reviewed and trustworthy
- [ ] zed_extension_api version is stable (0.7.0)
- [ ] npm mcp-searxng package is maintained
- [ ] License compatibility verified (all MIT-compatible)

### Privacy
- [ ] No telemetry or tracking
- [ ] No external requests except to configured SearXNG
- [ ] Privacy policy documented in README
- [ ] User data flow clearly explained

**Security Notes:**
```
[Any security concerns or recommendations]
```

---

## 7. Publishing Readiness ✅

### Repository Setup
- [ ] Repository is public on GitHub
- [ ] README.md is complete and renders correctly
- [ ] LICENSE file in repository root
- [ ] .gitignore excludes sensitive/build files
- [ ] Repository has clear description

### Metadata
- [ ] extension.toml has correct version
- [ ] extension.toml authors field populated
- [ ] extension.toml repository URL correct
- [ ] Cargo.toml version matches extension.toml
- [ ] All URLs in documentation work

### Git Hygiene
- [ ] All work committed
- [ ] Commit messages are descriptive
- [ ] No sensitive information in commits
- [ ] Repository tagged with version (e.g., v0.1.0)

### Pre-Publish Checklist
- [ ] All documentation reviewed and accurate
- [ ] CHANGELOG updated with release date
- [ ] GitHub release notes prepared
- [ ] Known issues documented (if any)

**GitHub Repository:**
```
URL: https://github.com/____________/mcp-server-searxng
Visibility: Public ☐ Private ☐
```

---

## 8. Zed Extension Submission ✅

### Extension Repository
- [ ] Forked zed-industries/extensions repository
- [ ] Added extension as git submodule (HTTPS URL)
- [ ] Updated extensions.toml with entry
- [ ] Ran `pnpm sort-extensions` successfully
- [ ] All changes committed

### Pull Request
- [ ] PR opened to zed-industries/extensions
- [ ] PR title follows format: "Add [extension-name] extension"
- [ ] PR description includes:
  - [ ] Extension description
  - [ ] Link to extension repository
  - [ ] Link to MCP server (mcp-searxng npm package)
  - [ ] Screenshots or demo (if applicable)
- [ ] PR passes CI checks

### Post-Submission
- [ ] Monitoring PR for reviewer feedback
- [ ] Ready to address review comments
- [ ] Plan for user support and bug reports

**PR URL:** ___________________________________________

---

## 9. User Testing ✅

### Beta Testing (Optional but Recommended)
- [ ] Shared with at least 3 beta testers
- [ ] Collected feedback on installation process
- [ ] Collected feedback on documentation clarity
- [ ] Collected feedback on functionality
- [ ] All critical bugs addressed

### Beta Tester Feedback:
```
Tester 1: [feedback]
Tester 2: [feedback]
Tester 3: [feedback]
```

---

## 10. Final Checks ✅

### Last-Minute Verification
- [ ] Re-read entire README start to finish
- [ ] Test installation from scratch in clean Zed
- [ ] Perform one complete search workflow
- [ ] Check all links in documentation one more time
- [ ] Verify version numbers are consistent everywhere

### Release Communications
- [ ] Announcement text prepared
- [ ] Community channels identified (Zed Discord, etc.)
- [ ] Support plan in place (GitHub issues)

### Post-Release Monitoring
- [ ] Plan to monitor GitHub issues daily for first week
- [ ] Plan to monitor Zed extension download metrics
- [ ] Plan to collect user feedback
- [ ] Rollback plan documented (if critical bug found)

**Communication Plan:**
```
[Where and when announcement will be made]
```

---

## Sign-Off

### All Critical Items Complete?
- [ ] YES - Ready to publish ✅
- [ ] NO - See issues below ❌

### Outstanding Issues (if any):
```
[List any blockers or concerns]
```

### Approvals

**Developer:** ___________________________ Date: __________

**Reviewer:** ___________________________ Date: __________

---

## Post-Release Checklist

Complete after extension is published:

- [ ] Monitor first 24 hours for critical bugs
- [ ] Respond to initial user questions
- [ ] Update README with any clarifications needed
- [ ] Document common user issues in troubleshooting
- [ ] Plan first maintenance update (bug fixes, improvements)

**First Week Metrics:**
```
Downloads: _____
Issues opened: _____
Stars: _____
User feedback: [summary]
```

---

## Notes

Use this space for any additional notes, observations, or recommendations:

```
[Additional notes]
```
