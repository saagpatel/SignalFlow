use signalflow_lib::db::Database;
use signalflow_lib::types::{FlowDocument, FlowNode};
use tempfile::TempDir;

#[test]
fn test_database_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let db = Database::open(&db_path);
    assert!(db.is_ok(), "Database should initialize successfully");

    // Verify WAL mode is enabled
    let db = db.unwrap();
    let conn = db.conn().unwrap();
    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .unwrap();
    assert_eq!(journal_mode.to_lowercase(), "wal", "WAL mode should be enabled");
}

#[test]
fn test_save_and_load_flow() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Create a test flow
    let flow = FlowDocument {
        id: Some("test-flow-1".to_string()),
        name: "Test Flow".to_string(),
        nodes: vec![
            FlowNode {
                id: "node-1".to_string(),
                node_type: "textInput".to_string(),
                position: serde_json::json!({"x": 0, "y": 0}),
                data: serde_json::json!({"value": "Hello"}),
            },
        ],
        edges: vec![],
    };

    // Save flow
    let saved_id = db.save_flow(&flow);
    assert!(saved_id.is_ok(), "Flow should save successfully");
    assert_eq!(saved_id.unwrap(), "test-flow-1");

    // Load flow
    let loaded = db.load_flow("test-flow-1");
    assert!(loaded.is_ok(), "Flow should load successfully");

    let loaded_flow = loaded.unwrap();
    assert_eq!(loaded_flow.id, Some("test-flow-1".to_string()));
    assert_eq!(loaded_flow.name, "Test Flow");
    assert_eq!(loaded_flow.nodes.len(), 1);
    assert_eq!(loaded_flow.nodes[0].id, "node-1");
}

#[test]
fn test_update_existing_flow() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Save initial flow
    let mut flow = FlowDocument {
        id: Some("flow-1".to_string()),
        name: "Original Name".to_string(),
        nodes: vec![],
        edges: vec![],
    };

    db.save_flow(&flow).unwrap();

    // Update flow
    flow.name = "Updated Name".to_string();
    db.save_flow(&flow).unwrap();

    // Verify update
    let loaded = db.load_flow("flow-1").unwrap();
    assert_eq!(loaded.name, "Updated Name");
}

#[test]
fn test_list_flows() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Save multiple flows
    for i in 1..=3 {
        let flow = FlowDocument {
            id: Some(format!("flow-{}", i)),
            name: format!("Flow {}", i),
            nodes: vec![],
            edges: vec![],
        };
        db.save_flow(&flow).unwrap();
    }

    // List flows
    let flows = db.list_flows();
    assert!(flows.is_ok());

    let flow_list = flows.unwrap();
    assert_eq!(flow_list.len(), 3, "Should have 3 flows");

    // Verify flows are ordered by updated_at DESC (most recent first)
    assert_eq!(flow_list[0].name, "Flow 3");
}

#[test]
fn test_delete_flow() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Save a flow
    let flow = FlowDocument {
        id: Some("delete-me".to_string()),
        name: "To Delete".to_string(),
        nodes: vec![],
        edges: vec![],
    };
    db.save_flow(&flow).unwrap();

    // Verify it exists
    assert!(db.load_flow("delete-me").is_ok());

    // Delete it
    let result = db.delete_flow("delete-me");
    assert!(result.is_ok(), "Delete should succeed");

    // Verify it's gone
    let load_result = db.load_flow("delete-me");
    assert!(load_result.is_err(), "Flow should no longer exist");
}

#[test]
fn test_delete_nonexistent_flow() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Try to delete a flow that doesn't exist
    let result = db.delete_flow("nonexistent");
    assert!(result.is_ok(), "Deleting nonexistent flow should not error");
}

#[test]
fn test_load_nonexistent_flow() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    let result = db.load_flow("nonexistent");
    assert!(result.is_err(), "Loading nonexistent flow should error");
}

#[test]
fn test_save_flow_auto_id() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    // Save flow without ID
    let flow = FlowDocument {
        id: None,
        name: "Auto ID Flow".to_string(),
        nodes: vec![],
        edges: vec![],
    };

    let saved_id = db.save_flow(&flow);
    assert!(saved_id.is_ok(), "Should auto-generate ID");

    let id = saved_id.unwrap();
    assert!(id.starts_with("flow_"), "Auto-generated ID should start with 'flow_'");

    // Verify can load by auto-generated ID
    let loaded = db.load_flow(&id);
    assert!(loaded.is_ok());
}

#[test]
fn test_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(Database::open(&db_path).unwrap());

    let mut handles = vec![];

    // Spawn 10 threads that each save a flow
    for i in 0..10 {
        let db_clone = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let flow = FlowDocument {
                id: Some(format!("concurrent-{}", i)),
                name: format!("Concurrent Flow {}", i),
                nodes: vec![],
                edges: vec![],
            };
            db_clone.save_flow(&flow)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        assert!(handle.join().unwrap().is_ok());
    }

    // Verify all flows were saved
    let flows = db.list_flows().unwrap();
    assert_eq!(flows.len(), 10, "All 10 flows should be saved");
}
