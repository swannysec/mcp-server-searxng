This MCP server requires access to a SearXNG instance.

## ⚠️ Security & Privacy Notice

**Credential Storage**: Authentication credentials are stored **unencrypted** in your Zed `settings.json` file and passed as environment variables (visible in process lists).

**Privacy**: Public SearXNG instances may log your search queries. For maximum privacy, self-host your own instance.

**SSRF Protection**: By default, localhost and private IP addresses are **allowed** (`allow_private_instances: true`) to support self-hosted SearXNG instances. Set to `false` only in untrusted/shared environments.

**Recommendation**: Use self-hosted instances **without authentication** when possible, or use a reverse proxy with authentication instead of HTTP Basic Auth.
</text>

<old_text line=20>
2. Configure the instance URL in your Zed settings (required)

**Optional:** Configure authentication, proxy settings, or custom User-Agent if needed (see security notice above).

## Setup Instructions

1. Choose a SearXNG instance:
   - Use a public instance from [searx.space](https://searx.space/) (check privacy policy)
   - **Recommended**: [Self-host your own](https://docs.searxng.org/admin/installation.html) for maximum privacy

2. Configure the instance URL in your Zed settings (required)

**Optional:** Configure authentication, proxy settings, or custom User-Agent if needed (see security notice above).