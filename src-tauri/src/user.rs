use crate::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub cards: Vec<Card>,
}

impl User {
    pub fn new() -> Self {
        return User { cards: vec![] };
    }
}
