mod commands;
mod db;
mod engine;
mod error;
mod nodes;
mod ollama;
mod sandbox;
mod state;
mod types;

use tauri::Manager;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data dir: {}", e))?;
            let db_path = app_dir.join("signalflow.db");

            let state = AppState::new(db_path)
                .map_err(|e| format!("Failed to initialize app state: {}", e))?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::execution::execute_flow,
            commands::execution::stop_execution,
            commands::flow::save_flow,
            commands::flow::load_flow,
            commands::flow::list_flows,
            commands::flow::delete_flow,
            commands::flow::get_execution_history,
            commands::node::get_node_definitions,
            commands::settings::get_preference,
            commands::settings::set_preference,
            commands::ollama::check_ollama,
            commands::ollama::list_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
