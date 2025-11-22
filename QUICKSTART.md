# Quick Start Guide

Get up and running with MCP Server: SearXNG in under 5 minutes.

## Prerequisites

- ✅ Zed Editor (v0.205.x or higher)
- ✅ Node.js 20+ installed
- ✅ Internet connection

## Step 1: Verify Node.js

```bash
node --version  # Should show v20.x.x or higher
npx --version   # Should show version number
```

If not installed: [Download Node.js](https://nodejs.org/)

## Step 2: Install Extension (Dev Mode)

1. **Clone or locate this repository**
   ```bash
   cd /path/to/mcp-server-searxng
   ```

2. **Build the extension**
   ```bash
   cargo build --target wasm32-wasip2 --release
   ```

3. **Install in Zed**
   - Open Zed
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
   - Type: `zed: install dev extension`
   - Select this directory: `mcp-server-searxng`
   - Wait for confirmation message

## Step 3: Configure

1. **Open Zed settings**
   - Press `Cmd+,` (macOS) or `Ctrl+,` (Windows/Linux)
   - Or: Settings menu → Open Settings

2. **Add this configuration**
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

3. **Save settings** (`Cmd+S` or `Ctrl+S`)

## Step 4: Test It Works

1. **Open AI Assistant in Zed**

2. **Start a conversation and try:**
   ```
   Search for "Rust async programming tutorial"
   ```

3. **Expected behavior:**
   - AI invokes `searxng_web_search` tool
   - Returns search results with titles and URLs
   - AI incorporates results into response

4. **Try URL reading:**
   ```
   Read and summarize: https://docs.searxng.org/
   ```

## Step 5: Verify Installation

### Check Zed Logs

1. Press `Cmd+Shift+P` and type: `zed: open log`
2. Look for:
   - ✅ No errors mentioning "mcp-server-searxng"
   - ✅ Extension loaded successfully
   - ✅ Context server connected

### Check Tools Available

In AI Assistant, the following tools should be available:
- `searxng_web_search` - Web search via SearXNG
- `web_url_read` - Fetch and convert URLs to markdown

## Common Quick Fixes

### "SEARXNG_URL not set" Error

**Fix:** Double-check your settings.json has the exact format above under `context_servers`

### "Command 'npx' not found"

**Fix:** Install Node.js 20+, restart terminal, restart Zed

### "Failed to connect to SearXNG"

**Fix:** Test the URL in browser first:
```bash
curl "https://searx.be/search?q=test&format=json"
```

If that fails, try a different instance from https://searx.space/

### Extension Not Appearing

**Fix:**
1. Check Zed version: `Zed → About Zed` (must be 0.205.x+)
2. Rebuild: `cargo clean && cargo build --target wasm32-wasip2 --release`
3. Reinstall: Remove extension, then `zed: install dev extension` again
4. Restart Zed completely

## Success Indicators

✅ **You're all set if:**
- AI Assistant responds to search queries
- Search results appear in conversations
- No errors in Zed logs
- `searxng_web_search` tool works

## Next Steps

Once working, explore:
- **Authentication**: Add `auth_username` and `auth_password` for private instances
- **Proxies**: Configure `http_proxy` and `https_proxy` if needed
- **Custom Instances**: Use your own self-hosted SearXNG

## Full Documentation

- **Complete Guide**: [README.md](README.md)
- **Testing**: [TESTING.md](TESTING.md)
- **Troubleshooting**: [README.md#troubleshooting](README.md#troubleshooting)
- **Configuration Examples**: [README.md#configuration-examples](README.md#configuration-examples)

## Need Help?

1. Check [README.md](README.md) troubleshooting section
2. Review [TESTING.md](TESTING.md) for detailed test cases
3. Open an issue: [GitHub Issues](https://github.com/yourusername/mcp-server-searxng/issues)
4. Join Zed Discord for community support

---

**Time to first search: ~3 minutes** ⚡

Got it working? Great! Now explore all the features in the [full README](README.md).