use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "mcp-searxng";
const SERVER_PATH: &str = "node_modules/mcp-searxng/dist/index.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-searxng";

struct SearxngModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct SearxngContextServerSettings {
    /// URL of the SearXNG instance (required)
    searxng_url: String,
    /// HTTP Basic Auth username (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_username: Option<String>,
    /// HTTP Basic Auth password (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_password: Option<String>,
    /// Custom User-Agent header (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,
    /// HTTP proxy URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    http_proxy: Option<String>,
    /// HTTPS proxy URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    https_proxy: Option<String>,
    /// Comma-separated list of hosts to bypass proxy (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<String>,
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
        // Check and install/update npm package
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        // Get node binary path
        let node_path = zed::node_binary_path()?;

        // Get the installed package path
        let server_path = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}. This may be due to permission issues or sandboxing restrictions.", e))?
            .join(SERVER_PATH)
            .to_string_lossy()
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

        // Validate searxng_url format
        if !settings.searxng_url.starts_with("http://") && !settings.searxng_url.starts_with("https://") {
            return Err(format!(
                "searxng_url must start with http:// or https://. Got: '{}'. Please update your Zed settings.",
                settings.searxng_url
            ));
        }

        // Build environment variables from settings
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
