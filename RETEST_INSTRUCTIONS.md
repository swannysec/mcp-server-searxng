# Retest Instructions - Critical Bug Fixed

## What Was Fixed

**Issue 1:** Extension loaded but MCP server didn't appear in Zed's Agent panel settings.

**Root Cause:** `extension.toml` was using incorrect `[mcp_servers.searxng]` format instead of `[context_servers.searxng]`.

**Fix Applied:** 
- Changed to correct `[context_servers.searxng]` format
- Removed command/args from toml (handled by Rust code)
- Committed in: `6f2e051 - fix: correct extension.toml to use context_servers format`

**Issue 2:** MCP server visible in Agent panel but won't start.

**Root Cause:** Rust code tried to read environment variables with `env::var()` in WASM context, but Zed doesn't set them there. Zed automatically injects settings from JSON schema when spawning the npx process.

**Fix Applied:**
- Removed all environment variable validation code
- Return empty env vec - Zed handles injection automatically
- Simplified to just return the npx command
- Committed in: `ba20782 - fix: remove env var validation - let Zed inject settings automatically`

---

## Steps to Reinstall and Retest

### Step 1: Rebuild the Extension

```bash
cd C:\Users\sabre\zed\mcp-server-searxng
cargo build --target wasm32-wasip2 --release
```

**Expected:** Build succeeds, WASM is 150KB

---

### Step 2: Reinstall in Zed

1. **Remove old extension:**
   - Open Zed
   - Go to Extensions panel
   - Find "MCP Server: SearXNG"
   - Uninstall or remove it

2. **Restart Zed completely** (important!)

3. **Reinstall:**
   - Press `Cmd+Shift+P` (or `Ctrl+Shift+P`)
   - Type: `zed: install dev extension`
   - Select directory: `C:\Users\sabre\zed\mcp-server-searxng`
   - Wait for confirmation

---

### Step 3: Verify Extension Loaded

1. Check Extensions panel - extension should be listed
2. Open logs: `Cmd+Shift+P` → `zed: open log`
3. Look for any errors (should be none)

---

### Step 4: Check Agent Panel Settings (NEW - This Should Work Now!)

1. Open Agent panel in Zed
2. Click the settings/gear icon in Agent panel
3. **YOU SHOULD NOW SEE:** "SearXNG Search" listed under context servers
4. It should show as available/enabled

**This is the critical fix - the context server should now appear!**

---

### Step 5: Verify Configuration

Your `settings.json` should still have:

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

**Important:** The settings key is `mcp-server-searxng` (the extension ID), even though the context server name in extension.toml is just `searxng`. This is correct.

---

### Step 6: Retest - Test 3 (Basic Search)

1. Open AI Assistant in Zed
2. Start a new conversation
3. Type: `Search for "Rust async programming best practices"`
4. Wait for response

**Expected Behavior:**
- MCP server should start (check Agent panel - should show as running/connected)
- AI should invoke `searxng_web_search` tool
- Search results should appear
- Results include titles, URLs, snippets
- Completes in < 10 seconds

---

## Quick Verification Checklist

- [ ] Extension rebuilt successfully (150KB WASM)
- [ ] Old extension uninstalled from Zed
- [ ] Zed restarted
- [ ] Extension reinstalled as dev extension
- [ ] No errors in Zed logs
- [ ] **"SearXNG Search" appears in Agent panel settings** ✨ (Fix 1)
- [ ] **MCP server starts successfully** ✨ (Fix 2)
- [ ] Configuration in settings.json is correct
- [ ] Test 3 (Basic Search) now passes
- [ ] AI can invoke `searxng_web_search` tool

---

## If It Still Doesn't Work

**Check the following:**

1. **Verify extension.toml:**
   ```bash
   cat extension.toml | grep "context_servers"
   ```
   Should show: `[context_servers.searxng]`

2. **Check Zed version:**
   - Must be 0.205.x or higher
   - Go to: Zed → About Zed

3. **Verify Node.js:**
   ```bash
   node --version  # Should be v20.x.x+
   npx --version
   ```

4. **Check Zed logs for specific errors:**
   - `Cmd+Shift+P` → `zed: open log`
   - Search for "searxng" or "mcp-server"
   - Copy any error messages

5. **Try different settings key:**
   Some users report needing to use just "searxng" instead of "mcp-server-searxng":
   ```json
   {
     "context_servers": {
       "searxng": {
         "settings": {
           "searxng_url": "https://searx.be"
         }
       }
     }
   }
   ```

---

## Expected Test Results

After fix, you should be able to report:

```
Test 1 (Installation): PASS ✅
Test 2 (Configuration): PASS ✅
Test 3 (Basic Search): PASS ✅  (Time: ___ sec)
Test 4 (URL Reading): [To be tested]
Test 5 (Error Handling): [To be tested]
Test 6 (Multiple Instances): [To be tested]
```

---

## Report Back

After retesting, please report:

1. **Does "SearXNG Search" now appear in Agent panel settings?** YES / NO
2. **Does MCP server start/connect successfully?** YES / NO (check status in Agent panel)
3. **Test 3 result:** PASS / FAIL
4. **If PASS:** Approximate search time in seconds
5. **If FAIL:** 
   - Error messages from logs
   - What specifically didn't work
   - Does server show as "starting" or "failed" in Agent panel?
   - Screenshots if helpful

---

## Next Steps

**If Test 3 now passes:**
- Continue with Tests 4, 5, and optionally 6
- Report all results when complete
- We can then finalize Phase 3 and move to Phase 4!

**If Test 3 still fails:**
- Provide the requested debugging information above
- I'll investigate further and provide another fix

---

**Estimated Time:** 10 minutes for reinstall + retest