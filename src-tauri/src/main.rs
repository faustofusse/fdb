// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn hola() -> String {
    return format!("<div><stong>HOLAAA</strong></div>")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, hola])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
