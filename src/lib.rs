use std::env;
use zed_extension_api::{self as zed, ContextServerId, Result};

/// MCP Server extension for SearXNG web search integration.
///
/// This extension provides a thin wrapper around the npm package `mcp-searxng`,
/// enabling privacy-focused web search capabilities for Zed's AI assistant.
///
/// The extension spawns the MCP server using npx, which executes the mcp-searxng
/// package without requiring a global installation.
struct McpServerSearxngExtension;

impl McpServerSearxngExtension {
    /// Validates that a SearXNG URL is properly formatted.
    fn validate_searxng_url(url: &str) -> Result<()> {
        if url.is_empty() {
            return Err("SEARXNG_URL cannot be empty. Please configure it in your Zed settings.json under context_servers.mcp-server-searxng.settings.searxng_url".into());
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(format!(
                "Invalid SEARXNG_URL: '{}'. URL must start with 'http://' or 'https://'. Example: 'https://searx.be'",
                url
            ));
        }

        if url.ends_with('/') {
            return Err(format!(
                "Invalid SEARXNG_URL: '{}'. URL should not end with a trailing slash. Use: '{}'",
                url,
                url.trim_end_matches('/')
            ));
        }

        Ok(())
    }

    /// Checks if Node.js and npx are available in the system PATH.
    fn check_nodejs_available() -> Result<()> {
        // We can't directly check for npx in WASM, but we can provide helpful error message
        // The actual check will happen when the command executes
        // This is a placeholder for documentation purposes
        Ok(())
    }

    /// Extracts settings from environment variables (set by Zed from settings.json)
    fn build_environment_variables() -> Result<Vec<(String, String)>> {
        let mut env_vars = Vec::new();

        // SEARXNG_URL is required
        if let Ok(searxng_url) = env::var("SEARXNG_URL") {
            Self::validate_searxng_url(&searxng_url)?;
            env_vars.push(("SEARXNG_URL".to_string(), searxng_url));
        } else {
            return Err("SEARXNG_URL environment variable not set.\n\n\
                Please add this to your Zed settings.json:\n\n\
                {\n\
                  \"context_servers\": {\n\
                    \"mcp-server-searxng\": {\n\
                      \"settings\": {\n\
                        \"searxng_url\": \"https://searx.be\"\n\
                      }\n\
                    }\n\
                  }\n\
                }\n\n\
                Find public instances at: https://searx.space/"
                .into());
        }

        // Optional: Authentication
        if let Ok(username) = env::var("AUTH_USERNAME") {
            env_vars.push(("AUTH_USERNAME".to_string(), username));
        }
        if let Ok(password) = env::var("AUTH_PASSWORD") {
            env_vars.push(("AUTH_PASSWORD".to_string(), password));
        }

        // Optional: User-Agent
        if let Ok(user_agent) = env::var("USER_AGENT") {
            env_vars.push(("USER_AGENT".to_string(), user_agent));
        }

        // Optional: Proxy settings
        if let Ok(http_proxy) = env::var("HTTP_PROXY") {
            env_vars.push(("HTTP_PROXY".to_string(), http_proxy));
        }
        if let Ok(https_proxy) = env::var("HTTPS_PROXY") {
            env_vars.push(("HTTPS_PROXY".to_string(), https_proxy));
        }
        if let Ok(no_proxy) = env::var("NO_PROXY") {
            env_vars.push(("NO_PROXY".to_string(), no_proxy));
        }

        Ok(env_vars)
    }
}

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
        // Validate and collect environment variables
        let env_vars = Self::build_environment_variables().map_err(|e| {
            format!(
                "Failed to configure MCP Server: SearXNG\n\n{}\n\n\
                For more help, see: https://github.com/yourusername/mcp-server-searxng#configuration",
                e
            )
        })?;

        // Check Node.js availability (informational)
        Self::check_nodejs_available()?;

        // Return the command to execute npx with the mcp-searxng package
        // The -y flag automatically confirms the package execution
        Ok(zed::Command {
            command: "npx".to_string(),
            args: vec!["-y".to_string(), "mcp-searxng".to_string()],
            env: env_vars,
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
