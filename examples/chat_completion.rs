use portkey_sdk::model::{
    ChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionUserMessageContent,
};
use portkey_sdk::service::ChatService;
use portkey_sdk::{AuthMethod, PortkeyClient, PortkeyConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client using environment variables or builder
    // Option 1: From environment (PORTKEY_API_KEY, PORTKEY_VIRTUAL_KEY, etc.)
    // let client = PortkeyClient::from_env()?;

    // Option 2: Using builder with virtual key
    let client = PortkeyConfig::builder()
        .with_api_key("your-portkey-api-key")
        .with_auth_method(AuthMethod::VirtualKey {
            virtual_key: "your-virtual-key".to_string(),
        })
        .build_client()?;

    // Option 3: Using provider authentication
    // let client = PortkeyConfig::builder()
    //     .with_api_key("your-portkey-api-key")
    //     .with_auth_method(AuthMethod::ProviderAuth {
    //         provider: "openai".to_string(),
    //         authorization: "Bearer sk-...".to_string(),
    //         custom_host: None,
    //     })
    //     .build_client()?;

    // Option 4: Using config-based routing
    // let client = PortkeyConfig::builder()
    //     .with_api_key("your-portkey-api-key")
    //     .with_auth_method(AuthMethod::Config {
    //         config_id: "pc-config-123".to_string(),
    //     })
    //     .build_client()?;

    // Create a chat completion request
    let request = ChatCompletionRequest {
        model: "gpt-4o".to_string(),
        messages: vec![
            ChatCompletionRequestMessage::System {
                content: "You are a helpful assistant.".to_string(),
                name: None,
            },
            ChatCompletionRequestMessage::User {
                content: ChatCompletionUserMessageContent::Text(
                    "What is the capital of France?".to_string(),
                ),
                name: None,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(100),
        frequency_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        n: None,
        presence_penalty: None,
        response_format: None,
        seed: None,
        stop: None,
        stream: None,
        stream_options: None,
        thinking: None,
        top_p: None,
        tools: None,
        tool_choice: None,
        parallel_tool_calls: None,
        user: None,
    };

    // Send the request
    let response = client.create_chat_completion(request).await?;

    // Print the response
    println!("Response ID: {}", response.id);
    println!("Model: {}", response.model);
    println!("Choices:");
    for (i, choice) in response.choices.iter().enumerate() {
        println!("  Choice {}:", i);
        println!("    Finish Reason: {}", choice.finish_reason);
        if let Some(content) = &choice.message.content {
            println!("    Content: {}", content);
        }
    }

    if let Some(usage) = response.usage {
        println!("\nUsage:");
        println!("  Prompt tokens: {}", usage.prompt_tokens);
        println!("  Completion tokens: {}", usage.completion_tokens);
        println!("  Total tokens: {}", usage.total_tokens);
    }

    Ok(())
}
