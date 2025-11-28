# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial implementation of Portkey Rust SDK
- `PortkeyConfig` and `PortkeyBuilder` for client configuration
- `PortkeyClient` with Arc-based cloning for multi-threaded usage
- Support for custom reqwest HTTP client
- Builder pattern support for configuration via `derive_builder`
- Environment variable configuration support
- Optional `tracing` feature for logging support
- Optional `strum` feature for enum string conversions
- Comprehensive error handling with `thiserror`

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

[Unreleased]: https://github.com/martsokha/portkey/compare/v0.1.0...HEAD
