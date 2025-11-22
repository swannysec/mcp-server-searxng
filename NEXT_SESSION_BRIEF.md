# Next Session Brief - MCP Server: SearXNG Extension

**Date:** November 22, 2024  
**Status:** Phase 3 Testing - BLOCKED on Bug 4  
**Platform:** Windows 11  
**Priority:** HIGH - Resolve environment variable passing issue

---

## Executive Summary

The mcp-server-searxng Zed extension is 95% complete. Through user testing, we discovered and fixed 3 critical bugs. A 4th bug is blocking completion: **environment variables from Zed settings are not reaching the spawned npm package on Windows**.

**Good News:**
- Extension loads successfully ✅
- Appears in Zed Agent panel ✅
- Configuration UI works ✅
- Spawns process without errors ✅
- Manual testing confirms npm package works correctly ✅

**The Problem:**
- MCP server times out after 60 seconds
- Error: `SEARXNG_URL not set`
- Settings ARE configured in Zed's settings.json
- Settings_schema is defined correctly
- Process spawns but doesn't receive environment variables

---

## Current State

### What's Working

1. **Extension Structure** ✅
   - `extension.toml` - Correct `[context_servers.searxng]` format
   - `Cargo.toml` - Builds successfully to WASM (150KB)
   - `src/lib.rs` - Clean, well-documented code
   - All documentation complete (3300+ lines)

2. **Zed Integration** ✅
   - Extension visible in extensions panel
   - Context server visible in Agent panel settings
   - Configuration dialog appears and accepts settings
   - No spawn errors in logs

3. **Manual Testing** ✅
   - Confirmed: `$env:SEARXNG_URL = "https://search.swannylab.net"; npx -y mcp-searxng` works perfectly
   - npm package (mcp-searxng) functions correctly when env var is set

### What's NOT Working

**Bug 4: Environment Variable Passing on Windows**

**Symptom:**
```
ERROR [project::context_server_store] searxng context server failed to start: Context server request timeout
```

**Root Cause:**
When Zed spawns the process, environment variables defined in `settings.json` (and validated by `settings_schema`) are not being passed to the child process.

**Current Implementation:**
```rust
Ok(zed::Command {
    command: "C:\\Program Files\\nodejs\\npx.cmd".to_string(),
    args: vec!["-y".to_string(), "mcp-searxng".to_string()],
    env: Vec::new(),  // ← Returning empty, expecting Zed to inject
})
```

**What We've Tried:**
1. ❌ `cmd.exe /C npx` - Env vars don't pass through shell
2. ❌ Direct `npx.cmd` call - Still no env vars received
3. ❌ Reading with `env::var()` in WASM - Not accessible in WASM context

---

## Bugs Fixed (For Reference)

### Bug 1: Wrong extension.toml Format
- **Issue:** Used `[mcp_servers.searxng]` instead of `[context_servers.searxng]`
- **Fix:** Changed to correct format - Commit `6f2e051`
- **Result:** Server now appears in Agent panel ✅

### Bug 2: Tried to Read Env Vars in WASM
- **Issue:** Used `env::var()` to read settings, which doesn't work in WASM
- **Fix:** Removed validation code, return empty env vec - Commit `ba20782`
- **Result:** Extension compiles and runs ✅

### Bug 3: Relative Command Paths
- **Issue:** Zed prepends extension directory to relative commands
- **Fix:** Use full absolute path `C:\\Windows\\System32\\cmd.exe` - Commit `239ac6e`
- **Result:** Command spawns successfully ✅

---

## Next Steps - Priority Order

### 🔴 Priority 1: Fix Environment Variable Passing (1-3 hours)

**Approach A: Use Wrapper Batch Script** (RECOMMENDED)
```batch
@echo off
REM wrapper.bat - Place in extension directory
set SEARXNG_URL=%1
set AUTH_USERNAME=%2
set AUTH_PASSWORD=%3
npx -y mcp-searxng
```

**Implementation:**
1. Create wrapper.bat in extension directory during build
2. Change command to call wrapper.bat with settings as arguments
3. Wrapper converts args to environment variables

**Pros:** 
- Explicit env var setting
- Can dynamically pass all settings
- Known pattern that works on Windows

**Cons:**
- Need to create/manage batch file
- Slightly more complex

---

**Approach B: PowerShell with Explicit Setting**
```rust
command: "powershell.exe",
args: vec![
    "-NoProfile",
    "-Command",
    "$env:SEARXNG_URL='[URL]'; npx -y mcp-searxng"
]
```

**Pros:**
- No additional files needed
- Explicit env var setting

**Cons:**
- Hardcodes URL (need to dynamically inject)
- May have PowerShell execution policy issues

---

**Approach C: Research Other Extensions**
- Search `zed-industries/extensions` repo for Windows MCP servers
- See how they handle environment variables
- Copy working pattern

**Action:** Check these extensions:
- mcp-server-brave-search
- mcp-server-github
- Any others with Windows support

---

**Approach D: Investigate Zed Source Code**
- Look at how Zed passes env vars on Windows
- Check if there's a bug or missing feature
- May need to file issue

**Files to check:**
- `crates/project/src/context_server_store.rs` (error source)
- How Command.env is processed on Windows

---

**Approach E: Try Populating Command.env Manually**

Since we can't read env vars in WASM, maybe we need to hardcode a test:

```rust
// TEST ONLY - hardcoded
env: vec![("SEARXNG_URL".to_string(), "https://searx.be".to_string())]
```

If this works, then we know the problem is that we can't ACCESS the settings, not that Zed doesn't pass them.

---

### 🟡 Priority 2: Complete Testing (15 minutes)

Once Bug 4 is fixed, complete these tests from USER_TESTING_REQUIRED.md:

- [ ] Test 3: Basic Search (currently failing)
- [ ] Test 4: URL Reading
- [ ] Test 5: Error Handling  
- [ ] Test 6: Multiple Instances (optional)

**Expected:** All tests should pass once env vars work.

---

### 🟢 Priority 3: Finalize Phase 3 (15 minutes)

- Update CHANGELOG.md with all fixes
- Update test results in ConPort
- Mark Phase 3 as DONE
- Create Phase 3 completion report

---

### 🔵 Priority 4: Phase 4 - Publishing (2-3 days)

Only start after Phase 3 complete:

1. Create public GitHub repository
2. Update all URLs in documentation (currently placeholder)
3. Update authors field in extension.toml
4. Tag v0.1.0
5. Fork zed-industries/extensions
6. Add as submodule
7. Submit PR
8. Wait for review

---

## Technical Reference

### Current File Structure
```
mcp-server-searxng/
├── .gitignore
├── AGENTS.md (project rules)
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── extension.toml ← Context server definition
├── LICENSE (MIT)
├── PRE_FLIGHT_CHECKLIST.md
├── QUICKSTART.md
├── README.md (348 lines)
├── RETEST_INSTRUCTIONS.md
├── src/
│   └── lib.rs ← Main extension code (73 lines)
├── TESTING.md (626 lines, 17 test cases)
└── USER_TESTING_REQUIRED.md
```

### Key Code - context_server_command()

**Location:** `src/lib.rs` lines 55-73

```rust
fn context_server_command(
    &mut self,
    _context_server_id: &ContextServerId,
    _project: &zed::Project,
) -> Result<zed::Command> {
    Ok(zed::Command {
        command: "C:\\Program Files\\nodejs\\npx.cmd".to_string(),
        args: vec![
            "-y".to_string(),
            "mcp-searxng".to_string(),
        ],
        env: Vec::new(),  // ← THE PROBLEM
    })
}
```

**The Issue:** `env: Vec::new()` means no environment variables are passed.

**What We Need:** Somehow populate this vec with:
```rust
vec![
    ("SEARXNG_URL".to_string(), "<from settings>".to_string()),
    // ... other optional settings
]
```

**The Challenge:** Can't access settings in WASM context.

---

### Key Code - context_server_configuration()

**Location:** `src/lib.rs` lines 75-168

Defines the settings_schema:
```json
{
  "properties": {
    "searxng_url": { "type": "string", "required": true },
    "auth_username": { "type": "string" },
    // ... etc
  }
}
```

**This works correctly** - Zed validates settings against this schema.

---

### User's Configuration

**File:** `~/.config/zed/settings.json` (or Windows equivalent)

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://search.swannylab.net"
      }
    }
  }
}
```

**This is correct** - Shows up in Zed configuration dialog.

---

## Git Status

**Branch:** main  
**Latest Commit:** `7406e7e - fix: call npx.cmd directly instead of through cmd.exe`  
**Total Commits:** 17  
**Working Tree:** Clean  

**Recent Commits (Most Recent First):**
```
7406e7e - fix: call npx.cmd directly instead of through cmd.exe
239ac6e - fix: use full path to cmd.exe instead of relative command
8058dea - fix: use cmd.exe wrapper for npx on Windows
ba20782 - fix: remove env var validation - let Zed inject settings automatically
6f2e051 - fix: correct extension.toml to use context_servers format
```

---

## ConPort Data Updated

All context has been saved to ConPort:

- **Active Context:** Updated with current state, blocking issue, test results
- **Decision D-10:** Summary of 4 bugs (3 fixed, 1 blocking)
- **Progress P-15:** BLOCKED status with details
- **TroubleshootingSteps:** Complete guide with 5 alternative approaches
- **ProjectStatus:** Current commit status, build status
- **NextSessionTasks:** Prioritized task list
- **ProjectGlossary:** "Windows-spawn-issue" term added

**To load context in next session:**
```
get_active_context
get_decisions (limit 5)
get_progress (status_filter: BLOCKED)
get_custom_data (category: TroubleshootingSteps)
get_custom_data (category: NextSessionTasks)
```

---

## Quick Start for Next Session

1. **Load Context:**
   - Read this file (NEXT_SESSION_BRIEF.md)
   - Load ConPort: `get_active_context`
   - Review `TroubleshootingSteps/bug_4_env_vars_not_passed`

2. **Start with Approach A (Wrapper Script):**
   - Create `wrapper.bat` in extension directory
   - Modify `context_server_command()` to call wrapper with args
   - Rebuild, reinstall, test

3. **If Approach A fails, try Approach C:**
   - Research other Windows MCP extensions
   - Copy working pattern

4. **Test Immediately:**
   - Rebuild: `cargo build --target wasm32-wasip2 --release`
   - Reinstall in Zed
   - Check logs for "SEARXNG_URL not set" - should be gone
   - Try basic search

---

## Success Criteria

**Bug 4 is fixed when:**
- ✅ No "SEARXNG_URL not set" error in mcp-searxng output
- ✅ MCP server initializes within 60 seconds
- ✅ No timeout errors in Zed logs
- ✅ Search query works in AI Assistant

**Phase 3 is complete when:**
- ✅ All 6 manual tests pass
- ✅ Extension works end-to-end on Windows
- ✅ Documentation updated with results

---

## Important Notes

- **Platform-Specific:** This is a Windows issue. macOS/Linux may need different approach.
- **Manual Test Confirmed:** The npm package WORKS when env var is set manually.
- **Not a Package Issue:** This is about HOW Zed passes env vars on Windows.
- **Zed Version:** User testing on Zed 0.205.x
- **Node.js Version:** 20.x installed at standard location

---

## Files to Reference

- `src/lib.rs` - Main extension code (needs env var fix)
- `USER_TESTING_REQUIRED.md` - Test procedures
- `RETEST_INSTRUCTIONS.md` - How to reinstall and test
- `TESTING.md` - Full test suite (17 cases)
- `CHANGELOG.md` - Update after fix

---

## Estimated Time to Complete

- **Fix Bug 4:** 1-3 hours (depending on approach)
- **Complete Testing:** 15 minutes
- **Finalize Phase 3:** 15 minutes
- **Total:** 2-4 hours to Phase 3 completion

Then Phase 4 (Publishing) can begin: 2-3 days including PR review.

---

## Contact/Handoff

- All work committed and pushed
- ConPort fully updated with context
- No uncommitted changes
- Ready for next developer to continue

**Good luck! The finish line is close.** 🚀