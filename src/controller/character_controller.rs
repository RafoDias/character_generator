use crate::models::Character;

use super::service::character_service;

pub fn save(character: &Character) -> Character {
    return character_service::save(character.clone());
}

pub fn remove(character: &Character) -> Character {
    return character_service::remove(character.clone());
}

pub fn get_by_name(name: &String) -> Character {
    let character: Character = Character {
        name: name.clone(),
        level: 0,
        class: crate::models::Class::Warrior,
        birthday: chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
    };

    return character_service::get(character);
}
