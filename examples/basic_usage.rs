//! Basic usage example of the Portkey SDK.
//!
//! This example demonstrates how to create and configure a Portkey client.
//!
//! # Running this example
//!
//! Set your API key as an environment variable:
//! ```bash
//! export PORTKEY_API_KEY=your-api-key
//! cargo run --example basic_usage --features tracing
//! ```
//!
//! # Tracing
//!
//! Enable tracing output:
//! ```bash
//! RUST_LOG=portkey_sdk=debug cargo run --example basic_usage --features tracing
//! ```

use portkey_sdk::{PortkeyClient, PortkeyConfig, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("Portkey SDK - Basic Usage Example\n");

    // Method 1: Create client from environment variable
    println!("1. Creating client from environment variable PORTKEY_API_KEY...");
    let client = PortkeyClient::from_env()?;
    println!("   Client created successfully: {:?}\n", client);

    // Method 2: Create client with custom configuration
    println!("2. Creating client with custom configuration...");
    let custom_client = PortkeyConfig::builder()
        .with_api_key("demo-api-key")
        .with_base_url("https://api.portkey.ai/v1")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;
    println!("   Client created successfully: {:?}\n", custom_client);

    // Method 3: Create client with custom reqwest client
    println!("3. Creating client with custom reqwest client...");
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()?;

    let client_with_custom_http = PortkeyConfig::builder()
        .with_api_key("demo-api-key")
        .with_client(http_client)
        .build_client()?;
    println!(
        "   Client created successfully: {:?}\n",
        client_with_custom_http
    );

    println!("All examples completed successfully!");

    Ok(())
}
