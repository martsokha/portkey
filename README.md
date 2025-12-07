# Portkey SDK

[![Crates.io](https://img.shields.io/crates/v/portkey-sdk?style=flat-square&color=black)](https://crates.io/crates/portkey-sdk)
[![Documentation](https://img.shields.io/docsrs/portkey-sdk?style=flat-square&color=black)](https://docs.rs/portkey-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/martsokha/portkey/build.yml?style=flat-square&color=black)](https://github.com/martsokha/portkey/actions)

A Rust client library for the [Portkey AI Gateway](https://portkey.ai/). This
SDK provides a type-safe, ergonomic interface for managing AI gateway
operations, chat completions, embeddings, images, audio, and analytics.

## Features

- **Complete API Coverage**: Support for all Portkey API endpoints
- **Type Safety**: Strongly typed models with comprehensive validation
- **Async/Await**: Built on modern async Rust with `tokio` and `reqwest`

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
use portkey_sdk::{AuthMethod, PortkeyConfig, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyConfig::builder()
        .with_api_key("your-portkey-api-key")
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: "your-virtual-key".to_string(),
        })
        .with_base_url("https://api.portkey.ai/v1")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;

    // Use the client for API calls...

    Ok(())
}
```

### Environment Variables

The SDK can be configured using environment variables:

| Variable                      | Required | Default                     | Description                                                  |
| ----------------------------- | -------- | --------------------------- | ------------------------------------------------------------ |
| `PORTKEY_API_KEY`             | Yes      | -                           | Your Portkey API key from [console](https://app.portkey.ai/) |
| `PORTKEY_BASE_URL`            | No       | `https://api.portkey.ai/v1` | Custom API base URL                                          |
| `PORTKEY_TIMEOUT_SECS`        | No       | `30`                        | Request timeout in seconds (max: 300)                        |
| `PORTKEY_VIRTUAL_KEY`         | No       | -                           | Virtual key for routing                                      |
| `PORTKEY_TRACE_ID`            | No       | -                           | Trace ID for request tracking                                |
| `PORTKEY_CACHE_NAMESPACE`     | No       | -                           | Cache namespace for response caching                         |
| `PORTKEY_CACHE_FORCE_REFRESH` | No       | `false`                     | Force refresh cached responses                               |

```rust,no_run
use portkey_sdk::{PortkeyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client: PortkeyClient = PortkeyClient::from_env()?;
    Ok(())
}
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

Enable comprehensive logging and tracing via the [`tracing`](https://crates.io/crates/tracing) crate. 
Tracing targets are defined in `lib.rs` for fine-grained control over log output:

```toml
portkey-sdk = { version = "0.1", features = ["tracing"] }
```

## Examples

The `examples/` directory contains comprehensive usage examples:

```bash
# Set your API key
export PORTKEY_API_KEY="your-api-key"
export PORTKEY_VIRTUAL_KEY="your-virtual-key"

# Run the basic usage example
cargo run --example basic_usage

# Run the chat completion example
cargo run --example chat_completion

# Run the embeddings example
cargo run --example embeddings
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
for details on how to submit pull requests, report issues, and contribute to the
project.

## License

This project is licensed under the MIT License - see the
[LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Portkey Documentation](https://portkey.ai/docs)
- [Portkey API Reference](https://portkey.ai/docs/api-reference)
- [Full API Documentation](https://docs.rs/portkey-sdk)
