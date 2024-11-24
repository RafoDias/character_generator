mod controller;
mod models;
mod tests;
mod utils;

use utils::cli;

fn main() {
    cli::cli_manager::start();

    //

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

    // println!(
    //     "{:?}",
    //     character_controller::get_by_name(&"Jennifer".to_string())
    // );

    // let returned_character: Character = character_controller::get_by_name("Isabelle");
    // println!("{:?}", returned_character);
    // let returned_character: Character = character_controller::get_by_name("Natã");
    // println!("{:?}", returned_character);
    // let returned_character: Character = character_controller::get_by_name("Alice");
    // println!("{:?}", returned_character);
}
