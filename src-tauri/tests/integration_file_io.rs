use signalflow_lib::engine::context::ExecutionContext;
use signalflow_lib::nodes::input::FileReadExecutor;
use signalflow_lib::nodes::output::FileWriteExecutor;
use signalflow_lib::nodes::NodeExecutor;
use signalflow_lib::types::NodeValue;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_file_read_write_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Write file
    let write_executor = FileWriteExecutor;
    let mut write_inputs = HashMap::new();
    write_inputs.insert(
        "path".to_string(),
        NodeValue::String(file_path.to_string_lossy().to_string()),
    );
    write_inputs.insert(
        "content".to_string(),
        NodeValue::String("Hello, Signalflow!".to_string()),
    );

    let write_config = serde_json::json!({
        "path": file_path.to_string_lossy(),
        "append": false
    });

    let ctx = ExecutionContext::new();
    let write_result = write_executor.execute(write_inputs, write_config, &ctx).await;
    assert!(write_result.is_ok(), "File write should succeed");

    // Read file back
    let read_executor = FileReadExecutor;
    let mut read_inputs = HashMap::new();
    read_inputs.insert(
        "path".to_string(),
        NodeValue::String(file_path.to_string_lossy().to_string()),
    );

    let read_config = serde_json::json!({
        "path": file_path.to_string_lossy()
    });

    let read_result = read_executor.execute(read_inputs, read_config, &ctx).await;
    assert!(read_result.is_ok(), "File read should succeed");

    if let Ok(outputs) = read_result {
        if let Some(NodeValue::String(content)) = outputs.get("content") {
            assert_eq!(content, "Hello, Signalflow!", "Content should match");
        } else {
            panic!("Expected string content");
        }
    }
}

#[tokio::test]
async fn test_file_append_mode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("append_test.txt");

    let executor = FileWriteExecutor;
    let ctx = ExecutionContext::new();

    // Write initial content
    let mut inputs1 = HashMap::new();
    inputs1.insert(
        "path".to_string(),
        NodeValue::String(file_path.to_string_lossy().to_string()),
    );
    inputs1.insert(
        "content".to_string(),
        NodeValue::String("Line 1\n".to_string()),
    );

    let config1 = serde_json::json!({
        "path": file_path.to_string_lossy(),
        "append": false
    });

    executor.execute(inputs1, config1, &ctx).await.unwrap();

    // Append more content
    let mut inputs2 = HashMap::new();
    inputs2.insert(
        "path".to_string(),
        NodeValue::String(file_path.to_string_lossy().to_string()),
    );
    inputs2.insert(
        "content".to_string(),
        NodeValue::String("Line 2\n".to_string()),
    );

    let config2 = serde_json::json!({
        "path": file_path.to_string_lossy(),
        "append": true
    });

    executor.execute(inputs2, config2, &ctx).await.unwrap();

    // Read and verify
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Line 1\nLine 2\n", "Should have both lines");
}

#[tokio::test]
async fn test_file_read_nonexistent() {
    let executor = FileReadExecutor;
    let mut inputs = HashMap::new();
    inputs.insert(
        "path".to_string(),
        NodeValue::String("/nonexistent/file/path.txt".to_string()),
    );

    let config = serde_json::json!({
        "path": "/nonexistent/file/path.txt"
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_err(), "Reading nonexistent file should fail");
}

#[tokio::test]
async fn test_path_traversal_prevention() {
    let executor = FileWriteExecutor;
    let mut inputs = HashMap::new();
    inputs.insert(
        "path".to_string(),
        NodeValue::String("../../../etc/passwd".to_string()),
    );
    inputs.insert(
        "content".to_string(),
        NodeValue::String("malicious".to_string()),
    );

    let config = serde_json::json!({
        "path": "../../../etc/passwd",
        "append": false
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    // Should reject paths with ".."
    assert!(result.is_err(), "Path traversal should be prevented");
}

#[tokio::test]
async fn test_large_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large_file.txt");

    // Create a 1MB string
    let large_content = "x".repeat(1_000_000);

    let executor = FileWriteExecutor;
    let mut inputs = HashMap::new();
    inputs.insert(
        "path".to_string(),
        NodeValue::String(file_path.to_string_lossy().to_string()),
    );
    inputs.insert(
        "content".to_string(),
        NodeValue::String(large_content.clone()),
    );

    let config = serde_json::json!({
        "path": file_path.to_string_lossy(),
        "append": false
    });

    let ctx = ExecutionContext::new();
    let result = executor.execute(inputs, config, &ctx).await;

    assert!(result.is_ok(), "Should handle large files");

    // Verify file size
    let metadata = fs::metadata(&file_path).unwrap();
    assert_eq!(metadata.len(), 1_000_000, "File size should be 1MB");
}
