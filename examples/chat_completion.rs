//! Chat completion example demonstrating authentication methods.
//!
//! This example shows how to use different authentication methods with the Portkey SDK.
//!
//! # Running this example
//!
//! ```bash
//! cargo run --example chat_completion --features tracing
//! ```
//!
//! # Tracing
//!
//! Enable tracing output:
//! ```bash
//! RUST_LOG=portkey_sdk=debug cargo run --example chat_completion --features tracing
//! ```

use portkey_sdk::model::{ChatCompletionRequest, ChatCompletionRequestMessage};
use portkey_sdk::service::ChatService;
use portkey_sdk::{AuthMethod, PortkeyConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Create client with virtual key authentication
    let client = PortkeyConfig::builder()
        .with_api_key("your-portkey-api-key")
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: "your-virtual-key".to_string(),
        })
        .build_client()?;

    // Create a chat completion request using the builder methods
    let mut request = ChatCompletionRequest::new(
        "gpt-4o",
        vec![
            ChatCompletionRequestMessage::system("You are a helpful assistant."),
            ChatCompletionRequestMessage::user("What is the capital of France?"),
        ],
    );
    request.temperature = Some(0.7);
    request.max_tokens = Some(100);

    // Send the request
    let response = client.create_chat_completion(request).await?;

    // Print the response
    println!(
        "Response: {}",
        response.choices[0].message.content.as_ref().unwrap()
    );

    if let Some(usage) = response.usage {
        println!("Tokens used: {}", usage.total_tokens);
    }

    Ok(())
}
