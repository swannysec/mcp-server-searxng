# SearXNG Search MCP Server

Privacy-focused web search for Zed AI assistant via SearXNG instances. Search the web and read URLs without compromising your privacy.

## Overview

This Zed extension provides [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server integration with [SearXNG](https://docs.searxng.org/), enabling your AI assistant to search the web and fetch webpage content while respecting your privacy.

**Key Features:**
- 🔍 **Web Search**: Search across multiple search engines via your SearXNG instance
- 📄 **URL Reading**: Fetch and convert web pages to markdown for AI analysis
- 🔒 **Privacy-First**: All searches go through YOUR configured SearXNG instance
- ⚡ **Fast & Lightweight**: 177KB WASM binary, minimal overhead
- 🛠️ **Flexible Configuration**: Support for authentication, proxies, and custom headers

## Requirements

- **Zed Editor**: Version 0.205.x or higher
- **Node.js**: Version 20 or higher (for npx)
- **SearXNG Instance**: Self-hosted or public instance access

## Installation

### 1. Install the Extension

**Option A: From Zed Extensions (Recommended)**
1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Search for "zed: extensions"
4. Search for "MCP Server: SearXNG"
5. Click "Install"

**Option B: As Dev Extension (For Development)**
1. Clone this repository
2. Open Zed
3. Press `Cmd+Shift+P` and search for "zed: install dev extension"
4. Select the extension directory

### 2. Find a SearXNG Instance

**Public Instances:**
Visit [searx.space](https://searx.space/) to find public SearXNG instances. Choose one with:
- ✅ JSON format enabled
- ✅ Good uptime
- ✅ Reasonable response time

**Self-Hosted:**
Follow the [SearXNG installation guide](https://docs.searxng.org/admin/installation.html) to host your own instance.

### 3. Configure in Zed

Open your Zed settings (`Cmd+,` or `Settings > Open Settings`):

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

### 4. Verify Installation

1. Open the AI Assistant panel in Zed
2. Start a new conversation
3. The assistant should now have access to `searxng_web_search` and `web_url_read` tools

> **Note:** Zed automatically converts settings from `settings.json` into environment variables that are passed to the MCP server. The extension validates these settings and provides helpful error messages if something is misconfigured.

## Configuration Examples

### Minimal Configuration (Public Instance)

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

### Authenticated SearXNG Instance

For password-protected instances:

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://your-private-instance.com",
        "auth_username": "your-username",
        "auth_password": "your-password"
      }
    }
  }
}
```

⚠️ **Security Note**: Credentials are stored in your local Zed settings file. Use environment-specific credentials, not personal passwords.

### Corporate Proxy Configuration

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://searx.be",
        "http_proxy": "http://proxy.company.com:8080",
        "https_proxy": "http://proxy.company.com:8080",
        "no_proxy": "localhost,127.0.0.1,.company.com"
      }
    }
  }
}
```

### Custom User-Agent

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://searx.be",
        "user_agent": "MyCustomBot/1.0"
      }
    }
  }
}
```

## Available Settings

| Setting | Type | Required | Description |
|---------|------|----------|-------------|
| `searxng_url` | string | ✅ Yes | URL of your SearXNG instance |
| `auth_username` | string | No | HTTP Basic Auth username |
| `auth_password` | string | No | HTTP Basic Auth password |
| `user_agent` | string | No | Custom User-Agent header |
| `http_proxy` | string | No | HTTP proxy URL |
| `https_proxy` | string | No | HTTPS proxy URL |
| `no_proxy` | string | No | Comma-separated hosts to bypass proxy |

## Usage Examples

Once configured, your AI assistant can use the search tools automatically:

**Example Queries:**
- "Search for Rust async best practices"
- "What are the latest features in Zed editor?"
- "Find documentation for zed_extension_api"
- "Read this URL and summarize: https://docs.searxng.org/"

The assistant will automatically invoke the appropriate tools and incorporate search results into its responses.

## Troubleshooting

### Error: "SEARXNG_URL environment variable not set"

**Cause**: Missing or incorrect configuration in Zed settings.

**Solution**:
1. Check your `settings.json` has the `context_servers.mcp-server-searxng.settings.searxng_url` key
2. Verify the URL format: `https://domain.com` (no trailing slash)
3. Restart Zed after making changes

### Error: "Command 'npx' not found"

**Cause**: Node.js or npx is not installed or not in PATH.

**Solution**:
1. Install Node.js 20+: https://nodejs.org/
2. Verify installation: `node --version` and `npx --version`
3. Restart Zed after installing Node.js

### Error: "Failed to connect to SearXNG instance"

**Cause**: SearXNG instance is down, URL is incorrect, or network issues.

**Solution**:
1. Test the URL in your browser: `https://your-instance.com`
2. Verify JSON format is enabled (visit `/search?q=test&format=json`)
3. Check firewall/proxy settings
4. Try a different public instance from [searx.space](https://searx.space/)

### Error: "Request timeout"

**Cause**: SearXNG instance is slow or unresponsive.

**Solution**:
1. Try a different SearXNG instance with better uptime
2. Self-host for better reliability and performance
3. Check your network connection

### Error: "401 Unauthorized"

**Cause**: Incorrect authentication credentials.

**Solution**:
1. Verify `auth_username` and `auth_password` are correct
2. Ensure your SearXNG instance requires Basic Auth (not all do)
3. Test credentials with curl:
   ```bash
   curl -u username:password https://your-instance.com/search?q=test&format=json
   ```

### Search results are empty or irrelevant

**Cause**: SearXNG instance configuration or selected search engines.

**Solution**:
1. Verify your SearXNG instance has search engines enabled
2. Visit your instance's settings and configure preferred engines
3. Self-host to have full control over search configuration

### Extension doesn't appear in AI assistant

**Cause**: Extension not loaded or Zed version incompatible.

**Solution**:
1. Check Zed version: Must be 0.205.x or higher
2. Restart Zed completely
3. Check Zed logs: `zed: open log` in command palette
4. Reinstall the extension

## Privacy & Security

### Data Flow

```
You → Zed → This Extension → mcp-searxng (npm) → Your SearXNG Instance → Search Engines
```

**What this extension does:**
- ✅ Wraps the `mcp-searxng` npm package
- ✅ Passes your settings as environment variables
- ✅ All data flows through YOUR configured SearXNG instance

**What this extension does NOT do:**
- ❌ Send data to any third-party servers (except your SearXNG instance)
- ❌ Log or store your search queries
- ❌ Include telemetry or analytics

**Your Responsibility:**
- Choose a trusted SearXNG instance or self-host
- Review the privacy policy of any public instance you use
- Keep your SearXNG instance credentials secure

## Development

### Building from Source

```bash
# Install Rust and add WASM target
rustup target add wasm32-wasip2

# Build
cargo build --target wasm32-wasip2 --release

# Check for issues
cargo clippy --target wasm32-wasip2 -- -D warnings

# Format code
cargo fmt
```

### Testing

See [TESTING.md](TESTING.md) for comprehensive testing procedures including:
- Manual testing guide with 17 test cases
- Validation tests for URL formats and configuration
- Integration tests for auth and proxy scenarios
- Performance and regression testing procedures
- Test report templates

### Testing Locally

```bash
# Install as dev extension in Zed
# zed: install dev extension (Cmd+Shift+P)

# View logs
# zed: open log (Cmd+Shift+P)

# Run validation tests
# See TESTING.md for detailed test cases
```

## Architecture

This extension follows the **Zed MCP Extension Wrapper Pattern**:

1. **Minimal Rust/WASM wrapper** - Implements Zed extension API
2. **Delegates to npm package** - Uses proven `mcp-searxng` implementation
3. **Configuration bridge** - Maps Zed settings to environment variables

This approach:
- ✅ Leverages battle-tested MCP server code
- ✅ Minimizes maintenance burden
- ✅ Keeps WASM binary small (147KB)
- ✅ Follows official Zed patterns

## Contributing

Contributions welcome! Please:

1. Read the code of conduct
2. Fork the repository
3. Create a feature branch
4. Make your changes with tests
5. Run `cargo clippy` and `cargo fmt`
6. Submit a pull request

## Roadmap

Future enhancements planned:

- [ ] Support for additional search engines (DuckDuckGo, Google Custom Search)
- [ ] Advanced result filtering
- [ ] Search history with privacy controls
- [ ] Image search support
- [ ] Configurable result limits and pagination

## License

This extension is licensed under the [MIT License](LICENSE).

## Links

- **Extension Repository**: https://github.com/yourusername/mcp-server-searxng
- **Zed Editor**: https://zed.dev/
- **SearXNG Documentation**: https://docs.searxng.org/
- **MCP Specification**: https://modelcontextprotocol.io/
- **npm mcp-searxng Package**: https://www.npmjs.com/package/mcp-searxng
- **Public SearXNG Instances**: https://searx.space/

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/mcp-server-searxng/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/mcp-server-searxng/discussions)
- **Zed Community**: [Zed Discord](https://discord.gg/zed)

## Acknowledgments

- [SearXNG Project](https://github.com/searxng/searxng) - Privacy-respecting metasearch engine
- [mcp-searxng](https://github.com/ihor-sokoliuk/mcp-searxng) - MCP server implementation
- [Zed Industries](https://zed.dev/) - High-performance code editor
- [Anthropic](https://www.anthropic.com/) - Model Context Protocol specification

---

**Made with ❤️ for privacy-conscious developers**