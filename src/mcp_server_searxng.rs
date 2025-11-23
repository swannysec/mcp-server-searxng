use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use url::Url;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "mcp-searxng";
const PACKAGE_VERSION: &str = "0.4.1"; // Pinned version for security - update after review
const SERVER_PATH: &str = "node_modules/mcp-searxng/dist/index.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-searxng";

struct SearxngModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct SearxngContextServerSettings {
    /// URL of the SearXNG instance (required)
    #[schemars(length(max = 2048))]
    searxng_url: String,
    /// HTTP Basic Auth username (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 256))]
    auth_username: Option<String>,
    /// HTTP Basic Auth password (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 256))]
    auth_password: Option<String>,
    /// Custom User-Agent header (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 256), regex(pattern = "^[a-zA-Z0-9 /_.-]+$"))]
    user_agent: Option<String>,
    /// HTTP proxy URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 2048))]
    http_proxy: Option<String>,
    /// HTTPS proxy URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 2048))]
    https_proxy: Option<String>,
    /// Comma-separated list of hosts to bypass proxy (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 1024))]
    no_proxy: Option<String>,
}

/// Validates a SearXNG instance URL for security
fn validate_searxng_url(url_str: &str) -> Result<()> {
    let url = Url::parse(url_str)
        .map_err(|e| format!("Invalid URL format: {}. Please provide a valid http:// or https:// URL.", e))?;

    // Reject non-HTTP(S) schemes
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(format!("URL must use http:// or https:// scheme. Got: {}", url.scheme()));
    }

    // Reject URLs with embedded credentials (security risk)
    if !url.username().is_empty() || url.password().is_some() {
        return Err("URLs with embedded credentials are not allowed. Use auth_username and auth_password settings instead.".to_string());
    }

    // Reject localhost/private IPs to prevent SSRF attacks
    if let Some(host) = url.host_str() {
        let host_lower = host.to_lowercase();
        if host_lower == "localhost"
            || host_lower.starts_with("127.")
            || host_lower.starts_with("192.168.")
            || host_lower.starts_with("10.")
            || host_lower.starts_with("172.16.")
            || host_lower.starts_with("172.17.")
            || host_lower.starts_with("172.18.")
            || host_lower.starts_with("172.19.")
            || host_lower.starts_with("172.20.")
            || host_lower.starts_with("172.21.")
            || host_lower.starts_with("172.22.")
            || host_lower.starts_with("172.23.")
            || host_lower.starts_with("172.24.")
            || host_lower.starts_with("172.25.")
            || host_lower.starts_with("172.26.")
            || host_lower.starts_with("172.27.")
            || host_lower.starts_with("172.28.")
            || host_lower.starts_with("172.29.")
            || host_lower.starts_with("172.30.")
            || host_lower.starts_with("172.31.")
            || host_lower == "0.0.0.0"
            || host_lower.starts_with("[::1]")
            || host_lower.starts_with("[::")
        {
            return Err(format!("Private/localhost URLs are not allowed for security reasons. Got: {}. Please use a publicly accessible SearXNG instance.", host));
        }
    }

    // Reject path traversal sequences
    if url.path().contains("..") {
        return Err("URL path contains path traversal sequences (..) which are not allowed for security reasons.".to_string());
    }

    Ok(())
}

/// Validates a User-Agent string for security
fn validate_user_agent(ua: &str) -> Result<()> {
    // Max length check
    if ua.len() > 256 {
        return Err(format!("User-Agent exceeds maximum length of 256 characters. Got: {} characters.", ua.len()));
    }

    // Character whitelist - alphanumeric plus common UA characters
    if !ua.chars().all(|c| c.is_alphanumeric() || " /-_.()".contains(c)) {
        return Err("User-Agent contains invalid characters. Only alphanumeric characters and ' /-_.()' are allowed.".to_string());
    }

    Ok(())
}

/// Validates a proxy URL for security
fn validate_proxy_url(url_str: &str) -> Result<()> {
    let url = Url::parse(url_str)
        .map_err(|e| format!("Invalid proxy URL format: {}. Please provide a valid http:// or https:// URL.", e))?;

    // Proxy URLs must be http or https
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(format!("Proxy URL must use http:// or https:// scheme. Got: {}", url.scheme()));
    }

    // Reject URLs with embedded credentials in proxy (pass separately if needed)
    if !url.username().is_empty() || url.password().is_some() {
        return Err("Proxy URLs with embedded credentials are not allowed for security reasons.".to_string());
    }

    // Reject path traversal sequences
    if url.path().contains("..") {
        return Err("Proxy URL path contains path traversal sequences (..) which are not allowed.".to_string());
    }

    Ok(())
}

/// Validates NO_PROXY bypass list for security
fn validate_no_proxy(no_proxy: &str) -> Result<()> {
    // Max length check
    if no_proxy.len() > 1024 {
        return Err(format!("NO_PROXY list exceeds maximum length of 1024 characters. Got: {} characters.", no_proxy.len()));
    }

    // Validate comma-separated list of hostnames
    for host in no_proxy.split(',').map(|s| s.trim()) {
        if host.is_empty() {
            continue;
        }

        // Simple hostname/pattern validation
        if !host.chars().all(|c| c.is_alphanumeric() || ".-*".contains(c)) {
            return Err(format!("Invalid hostname in NO_PROXY list: '{}'. Only alphanumeric characters and '.-*' are allowed.", host));
        }

        // Reject suspicious patterns
        if host.contains("..") {
            return Err(format!("NO_PROXY hostname '{}' contains suspicious '..' sequence.", host));
        }
    }

    Ok(())
}

impl zed::Extension for SearxngModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Check and install/update npm package (pinned version for security)
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        // Get node binary path
        let node_path = zed::node_binary_path()?;

        // Get the installed package path
        let server_path = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}. This may be due to permission issues or sandboxing restrictions.", e))?
            .join(SERVER_PATH);

        // Explicit UTF-8 validation instead of lossy conversion
        let server_path = server_path
            .to_str()
            .ok_or_else(|| "Server path contains invalid UTF-8 characters. Please ensure the extension directory path uses only ASCII characters.".to_string())?
            .to_string();

        // Read settings from Zed configuration
        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;

        // If no settings configured yet, return a minimal command
        // This allows Zed to show the configuration modal on first install
        let Some(settings_value) = settings.settings else {
            return Ok(Command {
                command: node_path,
                args: vec![server_path],
                env: Vec::new(),
            });
        };

        // Parse and validate settings
        let settings: SearxngContextServerSettings =
            serde_json::from_value(settings_value)
                .map_err(|e| format!("Invalid settings format: {}. Please check your searxng_url and optional fields in Zed settings.", e))?;

        // Validate searxng_url with comprehensive security checks
        validate_searxng_url(&settings.searxng_url)?;

        // Validate optional fields before using them
        if let Some(ref ua) = settings.user_agent {
            validate_user_agent(ua)?;
        }
        if let Some(ref proxy) = settings.http_proxy {
            validate_proxy_url(proxy)?;
        }
        if let Some(ref proxy) = settings.https_proxy {
            validate_proxy_url(proxy)?;
        }
        if let Some(ref no_proxy) = settings.no_proxy {
            validate_no_proxy(no_proxy)?;
        }

        // Build environment variables from validated settings
        let mut env_vars = vec![("SEARXNG_URL".into(), settings.searxng_url)];

        if let Some(username) = settings.auth_username {
            env_vars.push(("AUTH_USERNAME".into(), username));
        }
        if let Some(password) = settings.auth_password {
            env_vars.push(("AUTH_PASSWORD".into(), password));
        }
        if let Some(user_agent) = settings.user_agent {
            env_vars.push(("USER_AGENT".into(), user_agent));
        }
        if let Some(http_proxy) = settings.http_proxy {
            env_vars.push(("HTTP_PROXY".into(), http_proxy));
        }
        if let Some(https_proxy) = settings.https_proxy {
            env_vars.push(("HTTPS_PROXY".into(), https_proxy));
        }
        if let Some(no_proxy) = settings.no_proxy {
            env_vars.push(("NO_PROXY".into(), no_proxy));
        }

        Ok(Command {
            command: node_path,
            args: vec![server_path],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(SearxngContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(SearxngModelContextExtension);
