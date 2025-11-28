# Portkey SDK for Rust

A Rust SDK for interacting with the [Portkey](https://portkey.ai/) AI Gateway API.

## Features

- 🦀 **Idiomatic Rust** - Built with Rust best practices and type safety
- 🔧 **Flexible Configuration** - Builder pattern with environment variable support
- 🔒 **Thread-safe** - Clone-friendly client using `Arc` internally
- 🎯 **Custom HTTP Client** - Bring your own `reqwest::Client` for advanced configuration
- 📝 **Well Documented** - Comprehensive documentation and examples
- 🔍 **Optional Tracing** - Built-in observability support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
portkey-sdk = "0.1"
```

## Quick Start

### Basic Usage

```rust
use portkey_sdk::{PortkeyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client from environment variable PORTKEY_API_KEY
    let client = PortkeyClient::from_env()?;
    
    // Use the client...
    
    Ok(())
}
```

### Custom Configuration

```rust
use portkey_sdk::PortkeyConfig;
use std::time::Duration;

let client = PortkeyConfig::builder()
    .with_api_key("your-api-key")
    .with_base_url("https://api.portkey.ai/v1")
    .with_timeout(Duration::from_secs(60))
    .build_client()?;
```

### Custom HTTP Client

```rust
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

## Configuration

The SDK can be configured via:

1. **Environment Variables**:
   - `PORTKEY_API_KEY` - Your Portkey API key (required)
   - `PORTKEY_BASE_URL` - Custom API base URL (optional)
   - `PORTKEY_TIMEOUT_SECS` - Request timeout in seconds (optional, default: 30)

2. **Builder Pattern**:
   ```rust
   PortkeyConfig::builder()
       .with_api_key("...")
       .with_base_url("...")
       .with_timeout(Duration::from_secs(30))
       .build_client()?
   ```

## Features

### Default Features

- `rustls-tls` - Use rustls for TLS (enabled by default)

### Optional Features

- `native-tls` - Use native TLS instead of rustls
- `tracing` - Enable tracing support for observability
- `strum` - Enable string conversions for enums

## Examples

See the [examples](./examples) directory for more usage examples.

## License

MIT