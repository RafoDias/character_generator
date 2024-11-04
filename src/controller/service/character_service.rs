use crate::models::Character;

use super::repository::character_repository;

pub fn save(character: Character) {
    character_repository::save(character);
}

pub fn get(character: Character) -> Character {
    character_repository::get(character)
}

pub fn get_by_name(name: &str) -> Character {
    character_repository::get_by_name(name)
}
