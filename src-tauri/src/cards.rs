use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Hash, Serialize, Deserialize, Debug)]
pub struct Card {
    topic: String, // the cards' topic
    front: String, // the cards' front face, as a markdown string
    back: String,  // the cards' back face, as a markdown string
    bucket: u64,
}

impl Card {
    fn new() -> Self {
        return Card {};
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[command]
pub fn get_all_cards() -> Vec<Reminder> {
    return vec![Reminder::new(0, "test".into(), 0)];
}
