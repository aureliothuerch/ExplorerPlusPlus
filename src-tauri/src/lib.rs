use std::{fs};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PathFile {
    name: String,
    is_dir: bool,
}

#[tauri::command]
fn list_files_dir() -> Result<Vec<PathFile>, String> {
    let mut files_list: Vec<PathFile> = Vec::new();
    let path: String = r"C:\Projects\startup\Thuer".to_string();

    let files =  fs::read_dir(&path).map_err(|e| e.to_string())?;

    for file in files {
        if let Ok(de) = file {
            let file_name = de.file_name();
            let entry = file_name.to_string_lossy().into_owned();
            
            let path_file = PathFile {
                name: entry,
                is_dir: de.path().is_dir(),
            };

            files_list.push(path_file);
        }
        else {
            return Err("Failed to read a directory entry".to_string());
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
