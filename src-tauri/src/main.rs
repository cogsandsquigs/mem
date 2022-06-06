#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod reminders;

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![reminders::get_all_reminders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
