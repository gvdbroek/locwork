pub mod models;
pub use models::{DateLoc, generate_mock_data};
use serde::Serialize;
use std::fs;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_mock_data()->Vec<DateLoc>{
    // todo: replace this with less hard-coded path
    let fp = "/home/laika/code/locwork/mock/records.json";
    let file = fs::File::open(fp).expect("File should open read-only");
    let json : Vec<DateLoc> = serde_json::from_reader(file).unwrap();
    json
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_mock_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
