# USER TESTING REQUIRED

**Phase 3: Testing & Polish - Manual Testing Steps**

I've completed all automated checks and documentation for Phase 3. The following manual testing steps **require you to perform them in Zed** since I don't have access to run Zed or test the extension in a live environment.

---

## ✅ What I've Completed Autonomously

- ✅ Code quality validation (clippy, fmt, build)
- ✅ Created comprehensive testing documentation (TESTING.md)
- ✅ Created pre-flight checklist (PRE_FLIGHT_CHECKLIST.md)
- ✅ Created quick start guide (QUICKSTART.md)
- ✅ All code review and cleanup
- ✅ Documentation review and polish
- ✅ WASM build verification (150KB, within limits)
- ✅ All static validation complete

---

## ⚠️ REQUIRED: Manual Testing Steps

**You must complete these steps to verify the extension works correctly:**

### Test 1: Installation ✋ REQUIRED

**Time:** ~5 minutes

1. Ensure you have Node.js 20+ installed:
   ```bash
   node --version
   npx --version
   ```

2. In Zed, press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)

3. Type: `zed: install dev extension`

4. Select the directory: `C:\Users\sabre\zed\mcp-server-searxng`

5. Wait for confirmation message

**✅ SUCCESS CRITERIA:**
- Confirmation message appears
- No error messages in Zed logs
- Extension appears in Extensions list

**❌ IF IT FAILS:**
- Check Zed version (must be 0.205.x+)
- Run: `cargo build --target wasm32-wasip2 --release`
- Try again

---

### Test 2: Configuration ✋ REQUIRED

**Time:** ~2 minutes

1. Open Zed settings: `Cmd+,` or `Ctrl+,`

2. Add this configuration:
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

3. Save settings (`Cmd+S` or `Ctrl+S`)

4. Open Zed logs: `Cmd+Shift+P` → `zed: open log`

5. Check for errors

**✅ SUCCESS CRITERIA:**
- No errors in logs
- Extension loads successfully
- No "SEARXNG_URL not set" error

**❌ IF IT FAILS:**
- Check JSON syntax is correct
- Ensure `searxng_url` key matches exactly
- Restart Zed

---

### Test 3: Basic Search ✋ REQUIRED

**Time:** ~3 minutes

1. Open AI Assistant panel in Zed

2. Start a new conversation

3. Type: `Search for "Rust async programming best practices"`

4. Wait for response

5. Observe:
   - Does AI invoke `searxng_web_search` tool?
   - Do search results appear?
   - Are results relevant?
   - Time taken? (should be < 10 seconds)

**✅ SUCCESS CRITERIA:**
- Tool is invoked automatically
- Search completes in < 10 seconds
- Results include titles, URLs, snippets
- AI incorporates results into response

**❌ IF IT FAILS:**
- Check logs for error messages
- Test SearXNG URL in browser: `https://searx.be/search?q=test&format=json`
- Try different SearXNG instance from https://searx.space/

---

### Test 4: URL Reading ✋ REQUIRED

**Time:** ~2 minutes

1. In AI Assistant, type: `Read and summarize this URL: https://docs.searxng.org/`

2. Wait for response

3. Observe:
   - Does AI invoke `web_url_read` tool?
   - Is content fetched and converted?
   - Time taken? (should be < 15 seconds)

**✅ SUCCESS CRITERIA:**
- Tool is invoked
- Page content is read
- AI provides summary
- Completes in < 15 seconds

**❌ IF IT FAILS:**
- Check internet connectivity
- Check logs for specific errors
- Try a different URL

---

### Test 5: Error Handling ✋ REQUIRED

**Time:** ~3 minutes

**Test Invalid URL:**

1. Change settings to:
   ```json
   {
     "context_servers": {
       "mcp-server-searxng": {
         "settings": {
           "searxng_url": "searx.be"
         }
       }
     }
   }
   ```
   (Missing `https://`)

2. Save and check logs

3. **Expected:** Error message saying URL must start with 'http://' or 'https://'

**Test Missing URL:**

4. Change settings to:
   ```json
   {
     "context_servers": {
       "mcp-server-searxng": {
         "settings": {}
       }
     }
   }
   ```

5. Save and check logs

6. **Expected:** Error message with configuration example

**✅ SUCCESS CRITERIA:**
- Clear, helpful error messages
- Error includes configuration example
- No crash or hang

---

### Test 6: Multiple Instances (Optional but Recommended) 🎯

**Time:** ~5 minutes

Try at least 2 more public instances:

1. https://search.disroot.org
2. https://searx.tiekoetter.com

Update settings for each and perform a search.

**✅ SUCCESS CRITERIA:**
- Works with all instances
- Response times reasonable

---

## 📊 Test Results Report

**Please fill out after testing:**

```
Date Tested: _______________
Zed Version: _______________
Node.js Version: _______________
OS: _______________

Test 1 (Installation): PASS ☐  FAIL ☐
  Notes: _______________________________

Test 2 (Configuration): PASS ☐  FAIL ☐
  Notes: _______________________________

Test 3 (Basic Search): PASS ☐  FAIL ☐
  Search time: _______ seconds
  Notes: _______________________________

Test 4 (URL Reading): PASS ☐  FAIL ☐
  Read time: _______ seconds
  Notes: _______________________________

Test 5 (Error Handling): PASS ☐  FAIL ☐
  Notes: _______________________________

Test 6 (Multiple Instances): PASS ☐  FAIL ☐  SKIPPED ☐
  Instances tested: _______________________________
  Notes: _______________________________
```

---

## ✅ IF ALL TESTS PASS

**Phase 3 is complete!** 🎉

We can proceed to Phase 4: Publishing & Launch

Next steps I can do autonomously:
- Update CHANGELOG with test completion
- Mark Phase 3 tasks as DONE in ConPort
- Prepare Phase 4 checklist
- Create GitHub publishing guide

---

## ❌ IF ANY TESTS FAIL

**Please provide:**

1. Which test(s) failed
2. Exact error messages from logs
3. Screenshots if applicable
4. Any other relevant details

I'll help troubleshoot and fix any issues before proceeding.

---

## 📝 Quick Reference

**View logs:** `Cmd+Shift+P` → `zed: open log`

**Restart extension:** Restart Zed or AI Assistant

**Test SearXNG manually:**
```bash
curl "https://searx.be/search?q=test&format=json"
```

**Find other instances:** https://searx.space/

---

## Questions?

- See [QUICKSTART.md](QUICKSTART.md) for quick setup
- See [TESTING.md](TESTING.md) for comprehensive test cases
- See [README.md](README.md#troubleshooting) for troubleshooting

---

**Estimated Total Testing Time: 15-20 minutes**

Once you complete these tests and report back, I'll finalize Phase 3 and move to Phase 4 preparation! 🚀