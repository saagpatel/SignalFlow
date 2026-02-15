use signalflow_lib::engine::context::ExecutionContext;
use signalflow_lib::nodes::input::HttpRequestExecutor;
use signalflow_lib::nodes::NodeExecutor;
use signalflow_lib::types::NodeValue;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_http_get_request() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();

    let config = serde_json::json!({
        "url": "https://httpbin.org/get",
        "method": "GET",
        "headers": "{}"
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "HTTP GET request should succeed");

    if let Ok(outputs) = result {
        assert!(outputs.contains_key("response"), "Should have response output");
        assert!(outputs.contains_key("status"), "Should have status output");

        if let Some(NodeValue::Number(status)) = outputs.get("status") {
            assert_eq!(*status, 200.0, "Status should be 200");
        }
    }
}

#[tokio::test]
async fn test_http_post_request() {
    let executor = HttpRequestExecutor;
    let mut inputs = HashMap::new();
    inputs.insert("body".to_string(), NodeValue::String(r#"{"test": "data"}"#.to_string()));

    let config = serde_json::json!({
        "url": "https://httpbin.org/post",
        "method": "POST",
        "headers": r#"{"Content-Type": "application/json"}"#
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "HTTP POST request should succeed");
}

#[tokio::test]
async fn test_http_invalid_url() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();

    let config = serde_json::json!({
        "url": "http://this-domain-definitely-does-not-exist-12345.com",
        "method": "GET",
        "headers": "{}"
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_err(), "Invalid URL should return error");
}

#[tokio::test]
async fn test_http_timeout() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();

    // Use a local socket that accepts connections but delays response.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        if let Ok((_socket, _peer)) = listener.accept().await {
            sleep(Duration::from_secs(2)).await;
        }
    });

    let config = serde_json::json!({
        "url": format!("http://{}/slow", addr),
        "method": "GET",
        "headers": "{}",
        "timeoutMs": 100
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;
    server_task.abort();

    // Should timeout quickly based on timeoutMs.
    assert!(result.is_err(), "Request should timeout");
}

#[tokio::test]
async fn test_http_custom_headers() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();

    let config = serde_json::json!({
        "url": "https://httpbin.org/headers",
        "method": "GET",
        "headers": r#"{"X-Custom-Header": "test-value"}"#
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "Request with custom headers should succeed");

    if let Ok(outputs) = result {
        if let Some(NodeValue::String(response)) = outputs.get("response") {
            assert!(response.contains("X-Custom-Header"), "Response should include custom header");
        }
    }
}
