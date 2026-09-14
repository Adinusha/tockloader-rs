// src-tauri/src/lib.rs

#[tauri::command]
fn get_message(name: String) -> String {
    format!("Hello {name} from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register the command here:
        .invoke_handler(tauri::generate_handler![get_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}