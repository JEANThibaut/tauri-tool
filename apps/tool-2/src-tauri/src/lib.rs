use common::{AppConfig, format_message};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let config = AppConfig::new("Tool 2", "0.1.0");
    let message = format!("Bonjour {}! Bienvenue dans Tool 2!", name);
    format_message(&config.app_name, &message)
}

#[tauri::command]
fn get_app_info() -> AppConfig {
    AppConfig::new("Tool 2", "0.1.0")
}

// Nouvelle commande spécifique à Tool 2
#[tauri::command]
fn calculate(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_app_info, calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
