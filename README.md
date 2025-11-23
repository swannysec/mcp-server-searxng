# SearXNG Search MCP Server

Privacy-focused web search for Zed AI assistant via SearXNG instances. Search the web and read URLs without compromising your privacy.

## Overview

This Zed extension provides [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server integration with [SearXNG](https://docs.searxng.org/), enabling your AI assistant to search the web and fetch webpage content while respecting your privacy.

**Key Features:**
- 🔍 **Web Search**: Search across multiple search engines via your SearXNG instance
- 📄 **URL Reading**: Fetch and convert web pages to markdown for AI analysis
- 🔒 **Privacy-First**: All searches go through YOUR configured SearXNG instance
- ⚡ **Fast & Lightweight**: 133KB WASM binary, minimal overhead
- 🛠️ **Flexible Configuration**: Support for authentication, proxies, and custom headers

## ⚠️ Security & Privacy

**Important Security Considerations:**

### Credential Storage
- **Plaintext Storage**: Authentication credentials (`auth_username`, `auth_password`) are stored **unencrypted** in your Zed `settings.json` file
- **Process Visibility**: Credentials are passed as environment variables and may be visible in process lists (via `ps`, Task Manager, or similar tools)
- **Recommendation**: Use self-hosted SearXNG instances **without authentication** when possible, or use a reverse proxy with authentication instead of HTTP Basic Auth

### Privacy Considerations
- **Public Instances**: Public SearXNG instances (like `searx.be`) may log your search queries. Check the instance's privacy policy before use
- **Self-Hosted**: For maximum privacy, [self-host your own SearXNG instance](https://docs.searxng.org/admin/installation.html)
- **Instance Selection**: Visit [searx.space](https://searx.space/) to find instances with clear privacy policies and no-logging commitments

### SSRF Protection (Configurable)
- **Default Behavior**: Allows localhost and private IP addresses (`allow_private_instances: true`) to support self-hosted SearXNG instances
- **Strict Mode**: Set `allow_private_instances: false` to block localhost/RFC1918 private IPs and enforce SSRF protection
- **When to Use Strict Mode**: Only needed in shared/untrusted environments where malicious users might attempt SSRF attacks
- **Self-Hosted Users**: Keep the default `true` setting to access your local SearXNG instance

### Supply Chain Security
- This extension uses **pinned version 0.8.0** of the `mcp-searxng` npm package for security
- Updates require manual version changes and security review
- The extension validates all configuration inputs to prevent injection attacks

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

### Self-Hosted Configuration (Localhost)

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "http://localhost:8080",
        "allow_private_instances": true
      }
    }
  }
}
```

**Note:** `allow_private_instances` defaults to `true`, so you can omit it for localhost instances.

### Authenticated SearXNG Instance

⚠️ **Security Warning**: Credentials are stored in plaintext in `settings.json` and visible in process environment. Only use this with trusted, self-hosted instances.

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

### Strict SSRF Protection (Shared Environments)

If running in an untrusted environment where you need strict SSRF protection:

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://public-searxng-instance.com",
        "allow_private_instances": false
      }
    }
  }
}
```

**Note:** This blocks localhost and private IP addresses. Most users should **not** use this setting.

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

## Security Features

This extension implements multiple security controls:

- ✅ **URL Validation**: Comprehensive validation prevents URL injection, SSRF, and path traversal attacks
- ✅ **Input Sanitization**: All user inputs (User-Agent, proxy URLs, etc.) are validated with strict character whitelists
- ✅ **Version Pinning**: npm package version is pinned (0.8.0) to prevent supply chain attacks
- ✅ **Schema Constraints**: Maximum length limits on all string fields to prevent DoS
- ✅ **Configurable SSRF Protection**: Optional blocking of localhost/RFC1918 private IPs (disabled by default for self-hosted use)
- ✅ **No Unsafe Code**: Extension uses memory-safe Rust with zero unsafe blocks

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

## Contributing New Search Functionality

This extension is designed to be modular and extensible. If you'd like to add support for additional search engines, here's how to maintain consistency with the existing architecture:

### Architecture Overview

The extension follows a **wrapper pattern**:
1. **Rust WASM Extension** (`src/mcp_server_searxng.rs`) - Handles settings, validation, and npm package management
2. **npm MCP Server** (`mcp-searxng` package) - Implements the actual search logic and MCP protocol
3. **Settings Schema** - JSON schema for Zed's configuration UI

### Adding New Search Engines

To add a new search engine while maintaining architectural consistency:

1. **Fork the npm package** ([ihor-sokoliuk/mcp-searxng](https://github.com/ihor-sokoliuk/mcp-searxng))
2. **Implement the search provider** in TypeScript following the existing pattern:
   - Create a new provider module (e.g., `src/providers/duckduckgo.ts`)
   - Implement the same interface as existing providers (search query → JSON results)
   - Add configuration options to the MCP server settings
3. **Update the Rust wrapper**:
   - Add new settings fields to `SearxngContextServerSettings` struct
   - Add validation for new provider-specific settings
   - Update JSON schema with `#[schemars]` attributes
4. **Update documentation**:
   - Add configuration examples to README
   - Document new settings in `default_settings.jsonc`
   - Update `installation_instructions.md`

### Design Principles

- **Privacy-first**: All search providers should respect user privacy
- **Validation**: All user inputs must be validated in the Rust wrapper
- **Schema-driven**: Use JSON schema for type-safe configuration
- **Error handling**: Provide clear, actionable error messages
- **Testing**: Include examples and test cases

### Pull Request Guidelines

Contributions are welcome! Please:
- Follow the existing code style and patterns
- Include tests for new functionality
- Update documentation
- Ensure `cargo clippy` passes with no warnings
- Test with a real Zed installation before submitting

## License

This extension is licensed under the [MIT License](LICENSE).

## Links

- **Extension Repository**: https://github.com/swannysec/mcp-server-searxng
- **Zed Editor**: https://zed.dev/
- **SearXNG Documentation**: https://docs.searxng.org/
- **MCP Specification**: https://modelcontextprotocol.io/
- **npm mcp-searxng Package**: https://www.npmjs.com/package/mcp-searxng
- **Public SearXNG Instances**: https://searx.space/

## Support

- **Issues**: [GitHub Issues](https://github.com/swannysec/mcp-server-searxng/issues)
- **Discussions**: [GitHub Discussions](https://github.com/swannysec/mcp-server-searxng/discussions)
- **Zed Community**: [Zed Discord](https://discord.gg/zed)

## Acknowledgments

- [SearXNG Project](https://github.com/searxng/searxng) - Privacy-respecting metasearch engine
- [mcp-searxng](https://github.com/ihor-sokoliuk/mcp-searxng) - MCP server implementation by Ihor Sokoliuk
- [mcp-server-brave-search](https://github.com/zed-extensions/mcp-server-brave-search) - Reference implementation for Zed MCP extensions
- [Zed Industries](https://zed.dev/) - High-performance code editor
- [Anthropic](https://www.anthropic.com/) - Model Context Protocol specification

---

**Made with ❤️ for privacy-conscious developers**