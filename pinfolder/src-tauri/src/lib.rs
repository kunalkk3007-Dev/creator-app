mod commands;
mod fsutil;
mod model;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::pick_root_folder,
            commands::get_last_root,
            commands::list_projects,
            commands::create_project,
            commands::read_board,
            commands::write_board,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
