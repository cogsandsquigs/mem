use crate::card::Card;
use crate::user::User;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use tauri::command;

#[derive(Serialize, Deserialize, Clone)]
pub struct Database {
    path: &'static str,
    data: User,
}

impl Database {
    pub fn new(path: &'static str) -> Self {
        let mut db = Database {
            path,
            data: User::new(),
        };

        if std::path::Path::new(&db.path).exists() {
            let raw = fs::read_to_string(db.path).expect(&format!(
                "error encountered reading the file at {}",
                db.path
            ));

            db.data = serde_json::from_str(&raw).expect(&format!(
                "error encountered parsing data correctly, recieved {}",
                raw
            ));
        } else {
            let mut file = File::create(db.path).expect(&format!(
                "error encountered while creating file at {}",
                db.path
            ));
            file.write_all(
                &serde_json::to_vec(&db.data)
                    .expect("error encountered when serializing user data"),
            )
            .expect(&format!(
                "error encountered when writing user data to {}",
                db.path
            ));
        }

        return db;
    }

    fn write(&mut self) {
        let mut file = File::create(self.path).expect(&format!(
            "error encountered while creating file at {}",
            self.path
        ));
        file.write_all(
            &serde_json::to_vec(&self.data).expect("error encountered when serializing user data"),
        )
        .expect(&format!(
            "error encountered when writing user data to {}",
            self.path
        ));
    }

    pub fn add_card(&mut self, card: Card) {
        self.data.cards.append(&mut vec![card]);
        self.write();
        ()
    }
}

#[command]
pub fn get_new_db() -> Database {
    return Database::new("./data/db.json");
}
