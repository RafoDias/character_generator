mod controller;
mod models;
mod utils;

use chrono::NaiveDate;

use models::Character;
use models::Class;

use controller::character_controller;
use utils::cli;

fn main() {
    // let character: Character = Character {
    //     name: "Isabelle".to_string(),
    //     level: 1,
    //     class: Class::Rogue,
    //     birthday: NaiveDate::from_ymd_opt(2004, 04, 21).unwrap(),
    // };

    // character_controller::save(character);

    // let character: Character = Character {
    //     name: "Natã".to_string(),
    //     level: 1,
    //     class: Class::Warrior,
    //     birthday: NaiveDate::from_ymd_opt(2001, 11, 04).unwrap(),
    // };

    // character_controller::save(character);

    let returned_character: Character = character_controller::get_by_name("Isabelle");
    println!("{:?}", returned_character);
    let returned_character: Character = character_controller::get_by_name("Natã");
    println!("{:?}", returned_character);
    let returned_character: Character = character_controller::get_by_name("Alice");
    println!("{:?}", returned_character);
}
