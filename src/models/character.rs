use chrono::{self, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub class: super::Class,
    pub birthday: NaiveDate,
}

impl Character {
    pub fn new(name: &str, level: i32, class: super::Class, birthday: NaiveDate) -> Character {
        Character {
            name: name.to_string(),
            level,
            class,
            birthday,
        }
    }
}
