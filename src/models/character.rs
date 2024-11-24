use chrono::{self, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
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

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Character: {{\n\tName: {},\n\tLevel: {},\n\tClass: {},\n\tBirthday: {}\n}}",
            self.name, self.level, self.class, self.birthday
        )
    }
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Character: {{\n\tName: {},\n\tLevel: {},\n\tClass: {},\n\tBirthday: {}\n}}",
            self.name, self.level, self.class, self.birthday
        )
    }
}

pub fn get_empty_caracter() -> Character {
    return Character {
        name: "".to_string(),
        level: 0,
        class: super::Class::NoClass,
        birthday: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
    };
}
