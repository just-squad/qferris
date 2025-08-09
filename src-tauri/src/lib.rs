mod commands;
mod models;

use commands::{load_tabs, save_tabs, send_request};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let tabs_file = tauri::api::path::app_config_dir(&tauri::Config::default())
    //     .unwrap()
    //     .join("tabs.json");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .manage(AppState { tabs_file })
        .invoke_handler(tauri::generate_handler![save_tabs, load_tabs, send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
