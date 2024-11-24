use sled::IVec;

use crate::models::{character::get_empty_caracter, Character};

const REPO_PATH: &str = "/characters_db";

pub fn save(character: Character) -> Character {
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();
    let char = character.clone();
    let serialized_character: Vec<u8> = serde_cbor::to_vec(&char).unwrap();

    db.insert(&character.name, serialized_character).unwrap();
    let db_res = &db.get(&character.name).unwrap();

    if db_res.is_none() {
        return get_empty_caracter();
    }
    let serialized_value = db_res.clone().unwrap();

    return serde_cbor::from_slice(&serialized_value).unwrap();
}

pub fn get(character: Character) -> Character {
    let name: &str = &character.name;
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();

    let db_res = &db.get(name).unwrap();

    if db_res.is_none() {
        return get_empty_caracter();
    }
    let serialized_value = db_res.clone().unwrap();

    return serde_cbor::from_slice(&serialized_value).unwrap();
}

pub fn remove(character: Character) -> Character {
    let name: &str = &character.name;
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();

    let db_res = &db.remove(name).unwrap();
    if db_res.is_none() {
        return get_empty_caracter();
    }
    let serialized_value = db_res.clone().unwrap();

    return serde_cbor::from_slice(&serialized_value).unwrap();
}
