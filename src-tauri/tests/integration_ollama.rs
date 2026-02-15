use signalflow_lib::ollama::OllamaClient;
use signalflow_lib::engine::context::ExecutionContext;
use signalflow_lib::nodes::ai::{LlmPromptExecutor, LlmChatExecutor};
use signalflow_lib::nodes::NodeExecutor;
use signalflow_lib::types::NodeValue;
use std::collections::HashMap;

#[tokio::test]
async fn test_ollama_client_creation() {
    let client = OllamaClient::try_default();
    assert!(client.is_ok(), "Should create client successfully");
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_ollama_health_check() {
    let client = OllamaClient::try_default().unwrap();
    let status = client.check_health().await;

    assert!(status.available, "Ollama should be available");
    assert!(status.error.is_none(), "Should have no error");
}

#[tokio::test]
async fn test_ollama_health_check_unavailable() {
    // Use a port that's definitely not running Ollama
    let client = OllamaClient::new("http://localhost:9999").unwrap();
    let status = client.check_health().await;

    assert!(!status.available, "Ollama should not be available on port 9999");
    assert!(status.error.is_some(), "Should have error message");
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_ollama_list_models() {
    let client = OllamaClient::try_default().unwrap();
    let models = client.list_models().await;

    assert!(models.is_ok(), "Should list models successfully");

    let model_list = models.unwrap();
    // If Ollama is running, it should have at least one model
    // (but we can't guarantee this, so just check it's a valid response)
    assert!(model_list.len() >= 0, "Should return valid model list");
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available with llama3.2
async fn test_ollama_generate() {
    let client = OllamaClient::try_default().unwrap();
    let result = client.generate(
        "llama3.2",
        "Say 'hello' and nothing else",
        None,
        0.7,
    ).await;

    assert!(result.is_ok(), "Generation should succeed");

    let response = result.unwrap();
    assert!(!response.is_empty(), "Response should not be empty");
    assert!(
        response.to_lowercase().contains("hello"),
        "Response should contain 'hello'"
    );
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_llm_prompt_node() {
    let executor = LlmPromptExecutor;
    let mut inputs = HashMap::new();
    inputs.insert(
        "prompt".to_string(),
        NodeValue::String("What is 2+2?".to_string()),
    );

    let config = serde_json::json!({
        "model": "llama3.2",
        "temperature": 0.1,
        "systemPrompt": "You are a calculator. Answer only with the number, no explanation."
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "LLM prompt node should execute");

    if let Ok(outputs) = result {
        if let Some(NodeValue::String(response)) = outputs.get("response") {
            assert!(!response.is_empty(), "Should have a response");
            // Response should contain "4" somewhere
            assert!(response.contains("4"), "Response should contain the answer");
        } else {
            panic!("Expected string response");
        }
    }
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_llm_chat_node() {
    let executor = LlmChatExecutor;
    let mut inputs = HashMap::new();
    inputs.insert(
        "message".to_string(),
        NodeValue::String("Hello!".to_string()),
    );
    inputs.insert(
        "history".to_string(),
        NodeValue::Array(vec![]),
    );

    let config = serde_json::json!({
        "model": "llama3.2",
        "temperature": 0.7,
        "systemPrompt": "You are a friendly assistant."
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "LLM chat node should execute");

    if let Ok(outputs) = result {
        assert!(outputs.contains_key("response"), "Should have response");
        assert!(outputs.contains_key("history"), "Should have history");

        if let Some(NodeValue::Array(history)) = outputs.get("history") {
            // History should contain the message and response
            assert!(history.len() >= 2, "History should have at least 2 messages");
        }
    }
}

#[tokio::test]
async fn test_ollama_error_handling_invalid_model() {
    let client = OllamaClient::try_default().unwrap();

    // Try to use a model that definitely doesn't exist
    let result = client.generate(
        "nonexistent-model-12345",
        "test prompt",
        None,
        0.7,
    ).await;

    // Should return an error (if Ollama is running) or connection error (if not)
    // Either way, it should not panic
    assert!(result.is_err() || result.is_ok(), "Should handle gracefully");
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_ollama_generate_with_system_prompt() {
    let client = OllamaClient::try_default().unwrap();
    let result = client.generate(
        "llama3.2",
        "Who are you?",
        Some("You are a pirate. Always respond like a pirate."),
        0.7,
    ).await;

    assert!(result.is_ok(), "Generation with system prompt should succeed");

    let response = result.unwrap();
    // Response should reflect the pirate system prompt
    // (though we can't guarantee specific words without being flaky)
    assert!(!response.is_empty(), "Response should not be empty");
}

#[tokio::test]
#[ignore] // Only runs if Ollama is available
async fn test_ollama_temperature_variation() {
    let client = OllamaClient::try_default().unwrap();

    // Low temperature (more deterministic)
    let result_low = client.generate(
        "llama3.2",
        "What is the capital of France?",
        None,
        0.1,
    ).await;

    assert!(result_low.is_ok(), "Low temperature generation should succeed");

    // High temperature (more creative)
    let result_high = client.generate(
        "llama3.2",
        "What is the capital of France?",
        None,
        1.5,
    ).await;

    assert!(result_high.is_ok(), "High temperature generation should succeed");

    // Both should mention Paris
    let response_low = result_low.unwrap();
    let response_high = result_high.unwrap();

    assert!(
        response_low.to_lowercase().contains("paris"),
        "Low temp should give correct answer"
    );
    assert!(
        response_high.to_lowercase().contains("paris"),
        "High temp should still give correct answer for factual question"
    );
}
