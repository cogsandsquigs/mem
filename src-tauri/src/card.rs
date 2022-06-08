use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use tauri::command;

#[derive(Hash, Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    id: u64,
    topic: String,               // the cards' topic
    front: String,               // the cards' front face, as a markdown string
    back: String,                // the cards' back face, as a markdown string
    bucket: u64,                 // the bucket the card is in - used for the leitner system
    date_created: DateTime<Utc>, // the card's creation date
}

impl Card {
    pub fn new(topic: String, front: String, back: String, bucket: u64) -> Self {
        let mut card = Card {
            id: 0,
            topic,
            front,
            back,
            bucket,
            date_created: Utc::now(),
        };
        card.set_id();
        return card;
    }

    pub fn set_id(&mut self) {
        self.id = 0;
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        self.id = s.finish()
    }
}
