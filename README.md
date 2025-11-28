# Portkey SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/portkey-sdk?style=flat-square&color=black)](https://crates.io/crates/portkey-sdk)
[![Documentation](https://img.shields.io/docsrs/portkey-sdk?style=flat-square&color=black)](https://docs.rs/portkey-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/martsokha/portkey/build.yml?style=flat-square&color=black)](https://github.com/martsokha/portkey/actions)

A Rust client library for the [Portkey AI Gateway](https://portkey.ai/). This SDK provides a type-safe, ergonomic interface for managing AI gateway operations, analytics, and more.

## Features

- **Type Safety**: Strongly typed models with comprehensive validation
- **Async/Await**: Built on modern async Rust with `tokio` and `reqwest`
- **Flexible Configuration**: Builder pattern with environment variable support
- **Custom HTTP Client**: Bring your own `reqwest::Client` for advanced configuration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
portkey-sdk = { version = "0.1", features = [] }
```

## Quick Start

### Builder Configuration

```rust,no_run
use portkey_sdk::{PortkeyConfig, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyConfig::builder()
        .with_api_key("your-api-key")
        .with_base_url("https://api.portkey.ai/v1")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;

    // Use the client...

    Ok(())
}
```

### Environment Variables

The SDK can be configured using environment variables:

| Variable               | Required | Default                      | Description                          |
| ---------------------- | -------- | ---------------------------- | ------------------------------------ |
| `PORTKEY_API_KEY`      | Yes      | -                            | Your Portkey API key                 |
| `PORTKEY_BASE_URL`     | No       | `https://api.portkey.ai/v1`  | Custom API base URL                  |
| `PORTKEY_TIMEOUT_SECS` | No       | `30`                         | Request timeout in seconds (max: 300)|

```rust,no_run
use portkey_sdk::{PortkeyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyClient::from_env()?;
    Ok(())
}
```

### Custom HTTP Client

For advanced use cases, you can provide your own configured `reqwest::Client`:

```rust,no_run
use portkey_sdk::PortkeyConfig;
use reqwest::Client;
use std::time::Duration;

let custom_client = Client::builder()
    .timeout(Duration::from_secs(60))
    .pool_max_idle_per_host(10)
    .build()?;

let client = PortkeyConfig::builder()
    .with_api_key("your-api-key")
    .with_client(custom_client)
    .build_client()?;
```

## Optional Features

### TLS Backend

Choose between two TLS implementations:

```toml
# Default: rustls-tls (recommended)
portkey-sdk = { version = "0.1", features = [] }

# Alternative: native-tls
portkey-sdk = { version = "0.1", features = ["native-tls"], default-features = false }
```

### Tracing Support

Enable comprehensive logging and tracing:

```toml
portkey-sdk = { version = "0.1", features = ["tracing"] }
```

### Enum String Conversions

Enable string parsing and conversion for all enums:

```toml
portkey-sdk = { version = "0.1", features = ["strum"] }
```

## Examples

The `examples/` directory contains comprehensive usage examples:

```bash
# Set your API key
export PORTKEY_API_KEY="your-api-key"

# Run the basic usage example
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on how to submit pull requests, report issues, and contribute to the project.

## License

This project is licensed under the MIT License - see the [LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Portkey Documentation](https://portkey.ai/docs)
- [Full API Documentation](https://docs.rs/portkey-sdk)