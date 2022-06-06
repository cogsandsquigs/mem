use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    id: u64,      // the reminder's id
    name: String, // the reminder's name
    time: u64,    // the time the reminder goes off
}

impl Reminder {
    fn new(id: u64, name: String, time: u64) -> Self {
        return Reminder {
            id: id,
            name: name,
            time: time,
        };
    }
}

#[command]
pub fn get_all_reminders() -> Vec<Reminder> {
    return vec![Reminder::new(0, "test".into(), 0)];
}
