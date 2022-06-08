#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::Database;
use tauri::Manager;

pub mod card;
pub mod db;
pub mod user;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("add-card", |event| {
                let p = event.payload().unwrap();
                println!("got event-name with payload {:?}", p);
                let mut db = Database::new("./data/db.json");
                db.add_card(serde_json::from_slice(p.as_bytes()).unwrap())
            });
            // unlisten to the event using the `id` returned on the `listen_global` function
            // an `once_global` API is also exposed on the `App` struct
            // app.unlisten(id);
            // emit the `event-name` event to all webview windows on the frontend
            /*
            app.emit_all(
                "event-name",
                Card {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
            */
            Ok(())
        })
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![db::get_new_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
