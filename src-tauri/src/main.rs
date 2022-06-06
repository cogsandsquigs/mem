#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cards::Card;
use tauri::Manager;

mod cards;

fn main() {
    let mut card_store: Vec<String> = vec![];
    tauri::Builder::default()
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("add-card", |event| {
                println!("got event-name with payload {:?}", event.payload());
                //card_store.append(&mut vec![event.payload().unwrap().into()])
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
        .invoke_handler(tauri::generate_handler![cards::get_all_cards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
