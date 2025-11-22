use zed_extension_api::{self as zed, ContextServerId, Result};

/// MCP Server extension for SearXNG web search integration.
///
/// This extension provides a thin wrapper around the npm package `mcp-searxng`,
/// enabling privacy-focused web search capabilities for Zed's AI assistant.
///
/// The extension spawns the MCP server using npx, which executes the mcp-searxng
/// package without requiring a global installation.
struct McpServerSearxngExtension;

impl zed::Extension for McpServerSearxngExtension {
    /// Creates a new instance of the extension.
    fn new() -> Self {
        Self
    }

    /// Returns the command to start the MCP context server.
    ///
    /// This method is called by Zed when the AI assistant needs to initialize
    /// the SearXNG search context server. It configures the command to execute
    /// `npx -y mcp-searxng`, which automatically downloads and runs the latest
    /// version of the mcp-searxng npm package.
    ///
    /// # Configuration
    ///
    /// Users configure the extension via Zed's settings.json:
    ///
    /// ```json
    /// {
    ///   "context_servers": {
    ///     "mcp-server-searxng": {
    ///       "settings": {
    ///         "searxng_url": "https://searx.example.com",
    ///         "auth_username": "optional",
    ///         "auth_password": "optional",
    ///         "http_proxy": "optional",
    ///         "https_proxy": "optional"
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// # Environment Variables
    ///
    /// The mcp-searxng package expects these environment variables:
    /// - `SEARXNG_URL`: Required. URL of the SearXNG instance
    /// - `AUTH_USERNAME`: Optional. HTTP Basic Auth username
    /// - `AUTH_PASSWORD`: Optional. HTTP Basic Auth password
    /// - `USER_AGENT`: Optional. Custom User-Agent header
    /// - `HTTP_PROXY`: Optional. HTTP proxy URL
    /// - `HTTPS_PROXY`: Optional. HTTPS proxy URL
    /// - `NO_PROXY`: Optional. Comma-separated list of hosts to bypass proxy
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be constructed.
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command> {
        // Return the command to execute npx with the mcp-searxng package
        // The -y flag automatically confirms the package execution
        // Zed will automatically inject environment variables from the settings schema
        Ok(zed::Command {
            command: "npx".to_string(),
            args: vec!["-y".to_string(), "mcp-searxng".to_string()],
            env: Vec::new(),
        })
    }

    /// Returns configuration for the context server including settings schema.
    ///
    /// This provides Zed with the JSON schema for validating user settings
    /// and installation instructions for users.
    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<Option<zed::ContextServerConfiguration>> {
        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions: r#"
# SearXNG MCP Server

This extension requires:
- Node.js 20 or higher
- Access to a SearXNG instance (self-hosted or public)

## Quick Start

1. Find a SearXNG instance:
   - Use a public instance: https://searx.space/
   - Or self-host: https://docs.searxng.org/

2. Configure in Zed settings.json:

```json
{
  "context_servers": {
    "mcp-server-searxng": {
      "settings": {
        "searxng_url": "https://your-searxng-instance.com"
      }
    }
  }
}
```

## Optional Settings

- `auth_username` / `auth_password`: For password-protected instances
- `http_proxy` / `https_proxy`: For proxy configurations
- `user_agent`: Custom User-Agent header

For more information, visit: https://github.com/yourusername/mcp-server-searxng
"#
            .to_string(),
            settings_schema: r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "searxng_url": {
      "type": "string",
      "description": "URL of the SearXNG instance (required)",
      "format": "uri",
      "examples": ["https://searx.be", "https://search.disroot.org"]
    },
    "auth_username": {
      "type": "string",
      "description": "HTTP Basic Auth username (optional)"
    },
    "auth_password": {
      "type": "string",
      "description": "HTTP Basic Auth password (optional)"
    },
    "user_agent": {
      "type": "string",
      "description": "Custom User-Agent header (optional)"
    },
    "http_proxy": {
      "type": "string",
      "description": "HTTP proxy URL (optional)",
      "format": "uri"
    },
    "https_proxy": {
      "type": "string",
      "description": "HTTPS proxy URL (optional)",
      "format": "uri"
    },
    "no_proxy": {
      "type": "string",
      "description": "Comma-separated list of hosts to bypass proxy (optional)"
    }
  },
  "required": ["searxng_url"]
}"#
            .to_string(),
            default_settings: r#"{
  "searxng_url": "https://searx.be"
}"#
            .to_string(),
        }))
    }
}

// Register the extension with Zed
zed::register_extension!(McpServerSearxngExtension);
