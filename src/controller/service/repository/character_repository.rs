use crate::models::Character;

const REPO_PATH: &str = "/characters_db";

pub fn save(character: Character) -> Character {
    let name: &str = &character.name;
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();

    let serialized_character: Vec<u8> = serde_cbor::to_vec(&character).unwrap();
    db.insert(name, serialized_character).unwrap();

    let returned_character: Character =
        serde_cbor::from_slice(&db.get(name).unwrap().unwrap()).unwrap();
    return returned_character;
}

pub fn get(character: Character) -> Character {
    let name: &str = &character.name;
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();

    return serde_cbor::from_slice(&db.get(name).unwrap().unwrap()).unwrap();
}

pub fn get_by_name(name: &str) -> Character {
    let db: sled::Db = sled::open(super::get_database_path(REPO_PATH)).unwrap();

    return serde_cbor::from_slice(&db.get(name).unwrap().unwrap()).unwrap();
}
