// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{collections::HashMap, fs};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct PathFile {
    name: String,
    is_folder: bool,
}

#[tauri::command]
fn list_files_dir() -> Result<Vec<PathFile>, String> {
    let mut files_list: Vec<PathFile> = Vec::new();
    
    let files = fs::read_dir("C:/Projects/st/Thuer/").map_err(|e| e.to_string())?;
    for file in files {
        if let Ok(de) = file {
            let file_name = de.file_name();
            let entry = file_name.to_string_lossy().into_owned();
            
            let path_file = PathFile {
                name: entry,
                is_folder: true,
            };

            files_list.push(path_file);
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
