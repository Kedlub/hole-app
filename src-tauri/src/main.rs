// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;
#[cfg(any(windows, target_os = "macos"))]

fn main() {
    tauri::Builder::default().setup(|app| {
        let window = app.get_window("main").expect("Error");
        set_shadow(&window, true).unwrap(); 
        Ok(())
    }).invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
