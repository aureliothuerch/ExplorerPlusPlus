// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::Path;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_files_dir() -> Result<Vec<String>, String> {
    let mut files_list: Vec<String> = Vec::new();
    
    let files = fs::read_dir("./").map_err(|e| e.to_string())?;
    for file in files {
        if let Ok(de) = file {
            let file_name = de.file_name();
            let entry = file_name.to_string_lossy().into_owned();
            files_list.push(entry.to_string());
        }
    }

    Ok(files_list)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_files_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
