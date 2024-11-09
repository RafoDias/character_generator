use crate::models::Character;

use super::service::character_service;

pub fn save(character: Character) -> Character {
    return character_service::save(character);
}

pub fn get(character: Character) -> Character {
    character_service::get(character)
}

pub fn get_by_name(name: &str) -> Character {
    character_service::get_by_name(name)
}
