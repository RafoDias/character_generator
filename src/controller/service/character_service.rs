use crate::models::Character;

use super::repository::character_repository;

pub fn save(character: Character) -> Character {
    return character_repository::save(character);
}

pub fn get(character: Character) -> Character {
    return character_repository::get(character);
}

pub fn remove(character: Character) -> Character {
    return character_repository::remove(character);
}
