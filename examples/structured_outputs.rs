//! Structured outputs example using JSON Schema.
//!
//! This example demonstrates how to use the `schema` feature to generate
//! structured outputs from LLM responses.
//!
//! # Usage
//!
//! ```bash
//! export PORTKEY_API_KEY="your-api-key-here"
//! export PORTKEY_VIRTUAL_KEY="your-virtual-key-here"
//! cargo run --example structured_outputs --features schema
//! ```

use portkey_sdk::model::{ChatCompletionRequest, ChatCompletionRequestMessage, ResponseFormat};
use portkey_sdk::service::ChatService;
use portkey_sdk::{PortkeyClient, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct MovieRecommendation {
    title: String,
    year: u16,
    rating: f32,
    genre: String,
    reason: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = PortkeyClient::from_env()?;

    // Create a request with structured output using JSON Schema
    let mut request = ChatCompletionRequest::new(
        "gpt-4o",
        vec![ChatCompletionRequestMessage::user(
            "Recommend a great sci-fi movie from the 1980s",
        )],
    );

    // Configure the response format to use JSON Schema
    request.response_format = Some(ResponseFormat::JsonSchema {
        json_schema: ResponseFormat::json_schema::<MovieRecommendation>()
            .with_description("A movie recommendation with details")
            .with_strict(true),
    });

    let response = client.create_chat_completion(request).await?;

    // Deserialize the structured response
    if let Some(choice) = response.choices.first() {
        if let Some(movie) = choice
            .message
            .deserialize_content::<MovieRecommendation>()?
        {
            println!("\n🎬 Movie Recommendation:");
            println!("  Title: {}", movie.title);
            println!("  Year: {}", movie.year);
            println!("  Genre: {}", movie.genre);
            println!("  Rating: {:.1}/10", movie.rating);
            println!("  Reason: {}", movie.reason);
        }
    }

    if let Some(usage) = response.usage {
        println!("\nTokens used: {}", usage.total_tokens);
    }

    Ok(())
}
