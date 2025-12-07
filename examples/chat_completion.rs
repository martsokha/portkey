//! Chat completion example.
//!
//! # Usage
//!
//! ```bash
//! export PORTKEY_API_KEY="your-api-key-here"
//! export PORTKEY_VIRTUAL_KEY="your-virtual-key-here"
//! cargo run --example chat_completion
//! ```

use portkey_sdk::model::{ChatCompletionRequest, ChatCompletionRequestMessage};
use portkey_sdk::service::ChatService;
use portkey_sdk::{PortkeyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyClient::from_env()?;

    let request = ChatCompletionRequest::new(
        "gpt-4o",
        vec![
            ChatCompletionRequestMessage::system("You are a helpful assistant."),
            ChatCompletionRequestMessage::user("What is the capital of France?"),
        ],
    );

    let response = client.create_chat_completion(request).await?;

    println!(
        "Response: {}",
        response.choices[0].message.content.as_ref().unwrap()
    );

    if let Some(usage) = response.usage {
        println!("Tokens used: {}", usage.total_tokens);
    }

    Ok(())
}
