use signalflow_lib::engine::context::ExecutionContext;
use signalflow_lib::nodes::input::HttpRequestExecutor;
use signalflow_lib::nodes::NodeExecutor;
use signalflow_lib::types::NodeValue;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

async fn spawn_http_server(response: String) -> (String, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buf = [0u8; 4096];

        loop {
            let bytes_read = socket.read(&mut buf).await.unwrap();
            if bytes_read == 0 {
                break;
            }

            request.extend_from_slice(&buf[..bytes_read]);
            if request.windows(4).any(|chunk| chunk == b"\r\n\r\n") {
                break;
            }
        }

        socket.write_all(response.as_bytes()).await.unwrap();
        socket.shutdown().await.unwrap();

        String::from_utf8(request).unwrap()
    });

    (format!("http://{}", addr), server_task)
}

fn json_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}

#[tokio::test]
async fn test_http_get_request() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();
    let (base_url, server_task) = spawn_http_server(json_response("{\"ok\":true}\n")).await;

    let config = serde_json::json!({
        "url": format!("{base_url}/get"),
        "method": "GET",
        "headers": "{}"
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;
    let request = server_task.await.unwrap();

    assert!(result.is_ok(), "HTTP GET request should succeed");
    assert!(request.starts_with("GET /get HTTP/1.1"));

    if let Ok(outputs) = result {
        assert!(
            outputs.contains_key("response"),
            "Should have response output"
        );
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
    inputs.insert(
        "body".to_string(),
        NodeValue::String(r#"{"test":"data"}"#.to_string()),
    );
    let (base_url, server_task) = spawn_http_server(json_response("{\"saved\":true}\n")).await;

    let config = serde_json::json!({
        "url": format!("{base_url}/post"),
        "method": "POST",
        "headers": r#"{"Content-Type": "application/json"}"#
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;
    let request = server_task.await.unwrap();

    assert!(result.is_ok(), "HTTP POST request should succeed");
    assert!(request.starts_with("POST /post HTTP/1.1"));
    assert!(request.contains("content-type: application/json"));
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

    assert!(result.is_err(), "Request should timeout");
}

#[tokio::test]
async fn test_http_custom_headers() {
    let executor = HttpRequestExecutor;
    let inputs = HashMap::new();
    let (base_url, server_task) =
        spawn_http_server(json_response("{\"headers\":\"captured\"}\n")).await;

    let config = serde_json::json!({
        "url": format!("{base_url}/headers"),
        "method": "GET",
        "headers": r#"{"X-Custom-Header": "test-value"}"#
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;
    let request = server_task.await.unwrap();

    assert!(result.is_ok(), "Request with custom headers should succeed");
    assert!(request.contains("x-custom-header: test-value"));
}
