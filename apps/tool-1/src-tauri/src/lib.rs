use common::{AppConfig, format_message};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let config = AppConfig::new("Tool 1", "0.1.0");
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
    format_message(&config.app_name, &message)
}

#[tauri::command]
fn get_app_info() -> AppConfig {
    AppConfig::new("Tool 1", "0.1.0")
}

#[tauri::command]
fn test_only_function()-> String {
    // This function is only compiled and used during tests
    "This is a test-only function".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_app_info, test_only_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
