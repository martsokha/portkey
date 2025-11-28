use portkey_sdk::model::{CreateResponseRequest, ListInputItemsParams};
use portkey_sdk::{AuthMethod, PortkeyConfig, ResponsesService, Result};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client with your Portkey API key and auth method
    let client = PortkeyConfig::builder()
        .with_api_key(
            std::env::var("PORTKEY_API_KEY").unwrap_or_else(|_| "your-portkey-api-key".to_string()),
        )
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: std::env::var("PORTKEY_VIRTUAL_KEY")
                .unwrap_or_else(|_| "your-virtual-key".to_string()),
        })
        .build_client()?;

    println!("Creating a new response...");

    // Create metadata for the response
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), serde_json::json!("user123"));
    metadata.insert("environment".to_string(), serde_json::json!("production"));

    // Create a new response
    let create_request = CreateResponseRequest {
        trace_id: Some("trace-example-123".to_string()),
        metadata: Some(metadata),
        model: Some("gpt-4".to_string()),
        provider: Some("openai".to_string()),
        status: Some("success".to_string()),
        request: Some(serde_json::json!({
            "model": "gpt-4",
            "messages": [
                {"role": "user", "content": "Hello, how are you?"}
            ]
        })),
        response: Some(serde_json::json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "I'm doing well, thank you! How can I help you today?"
                },
                "finish_reason": "stop"
            }]
        })),
        total_tokens: Some(150),
        prompt_tokens: Some(100),
        completion_tokens: Some(50),
        latency_ms: Some(1500),
        cost: Some(0.003),
    };

    let response = client.create_response(create_request).await?;
    println!("✓ Created response with ID: {}", response.id);
    println!(
        "  - Model: {}",
        response.model.as_ref().unwrap_or(&"N/A".to_string())
    );
    println!(
        "  - Provider: {}",
        response.provider.as_ref().unwrap_or(&"N/A".to_string())
    );
    println!("  - Total tokens: {}", response.total_tokens.unwrap_or(0));
    println!("  - Cost: ${:.4}", response.cost.unwrap_or(0.0));

    // Retrieve the response
    println!("\nRetrieving the response...");
    let fetched = client.get_response(&response.id).await?;
    println!("✓ Retrieved response: {}", fetched.id);
    println!(
        "  - Status: {}",
        fetched.status.as_ref().unwrap_or(&"N/A".to_string())
    );
    println!("  - Latency: {}ms", fetched.latency_ms.unwrap_or(0));

    // List input items for the response
    println!("\nListing input items...");
    let params = ListInputItemsParams {
        limit: Some(50),
        offset: Some(0),
    };
    let input_items = client.list_input_items(&response.id, params).await?;
    println!("✓ Found {} input items", input_items.data.len());
    if let Some(total) = input_items.total {
        println!("  - Total available: {}", total);
    }

    for (i, item) in input_items.data.iter().enumerate() {
        println!("  - Item {}: {}", i + 1, item.id);
        if let Some(role) = &item.role {
            println!("    Role: {}", role);
        }
        if let Some(content) = &item.content {
            println!("    Content: {}", content);
        }
    }

    // Delete the response
    println!("\nDeleting the response...");
    client.delete_response(&response.id).await?;
    println!("✓ Successfully deleted response: {}", response.id);

    Ok(())
}
