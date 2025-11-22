# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial Zed extension implementation for MCP Server: SearXNG
- Core MCP server wrapper using Rust/WASM
- `context_server_command()` method to launch mcp-searxng via npx
- `context_server_configuration()` method with JSON schema for settings validation
- URL validation for SEARXNG_URL configuration
- Environment variable handling for all mcp-searxng settings:
  - `SEARXNG_URL` (required) - SearXNG instance URL
  - `AUTH_USERNAME` / `AUTH_PASSWORD` (optional) - HTTP Basic Auth
  - `USER_AGENT` (optional) - Custom User-Agent header
  - `HTTP_PROXY` / `HTTPS_PROXY` / `NO_PROXY` (optional) - Proxy configuration
- Comprehensive error messages with configuration examples
- Detailed installation instructions in README
- Configuration examples for common scenarios (basic, auth, proxy)
- Troubleshooting guide with solutions for common issues
- Privacy and security documentation
- Development and architecture documentation
- Comprehensive testing guide (TESTING.md) with 17 test cases
- MIT License
- Git repository with proper .gitignore

### Changed
- N/A (initial release)

### Fixed
- Clippy warnings for useless type conversions

### Technical Details
- Extension ID: `mcp-server-searxng`
- Target: `wasm32-wasip2`
- Zed Extension API: `0.7.0`
- WASM binary size: ~150KB (well under 500KB target)
- Compatible with Zed 0.205.x and higher
- Requires Node.js 20+ for npx execution

## [0.1.0] - TBD

Initial release planned after Phase 3 (Testing & Polish) and Phase 4 (Publishing) are complete.

### Release Criteria
- [ ] All manual tests passing (17 test cases in TESTING.md)
- [ ] End-to-end testing with real SearXNG instances
- [ ] No clippy warnings
- [ ] Complete documentation
- [ ] Performance validation (search < 10s, URL read < 15s)
- [ ] Successfully tested with at least 3 public SearXNG instances

---

## Development Phases

### Phase 1: Foundation & Setup ✅
**Status:** Complete  
**Commits:** 
- `0a44ea0` - Initialize repository with .gitignore
- `1fe1484` - Implement core MCP server extension
- `80e0922` - Add comprehensive README and MIT LICENSE
- `5d88d64` - Add Cargo.lock for reproducible builds

### Phase 2: Core Integration ✅
**Status:** Complete  
**Commits:**
- `dd4f6e5` - Add validation, error handling, and testing guide
- `ef5d54a` - Update README with testing reference and env var clarification

**Deliverables:**
- ✅ Working context_server_command() implementation
- ✅ Environment variable passing from Zed settings
- ✅ URL validation logic with helpful errors
- ✅ Error messages for configuration issues
- ✅ Comprehensive testing documentation

### Phase 3: Testing & Polish 🔄
**Status:** Autonomous Tasks Complete - Manual Testing Required  
**Commits:**
- `c39d8e7` - Add Phase 3 testing documentation and user guides

**Completed Autonomously:**
- ✅ Code quality validation (clippy, fmt, build - all passing)
- ✅ Documentation review and polish
- ✅ Code review and cleanup (no issues found)
- ✅ Created PRE_FLIGHT_CHECKLIST.md (10 comprehensive sections)
- ✅ Created QUICKSTART.md (5-minute setup guide)
- ✅ Created USER_TESTING_REQUIRED.md (6 mandatory test cases)
- ✅ WASM build verification (150KB, within limits)
- ✅ All static validation complete

**Requires Manual Testing:**
- ⏳ End-to-end testing with real SearXNG instances in Zed
- ⏳ Performance validation (search < 10s, URL read < 15s)
- ⏳ Configuration example testing in live environment
- ⏳ User acceptance testing with actual AI Assistant

**Blocked By:** User must perform manual tests in Zed (see USER_TESTING_REQUIRED.md)

### Phase 4: Publishing & Launch 📋
**Status:** Planned  
**Planned:**
- Final pre-release checks
- Create GitHub release
- Submit PR to zed-industries/extensions
- Prepare announcement

---

## Contributing

For contribution guidelines, see [README.md](README.md#contributing).

## Links

- **Repository**: https://github.com/yourusername/mcp-server-searxng
- **Issues**: https://github.com/yourusername/mcp-server-searxng/issues
- **Zed Extensions**: https://zed.dev/extensions