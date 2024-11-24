#[cfg(test)]
pub mod tests {
    use crate::controller::character_controller;
    use crate::models::*;
    use character::get_empty_caracter;
    use chrono::NaiveDate;

    fn get_const_test_character() -> Character {
        return Character {
            name: "teste".to_string(),
            level: 1,
            class: Class::Rogue,
            birthday: NaiveDate::from_ymd_opt(2004, 04, 21).unwrap(),
        };
    }

    #[test]
    pub fn complete_character_cycle() {
        println!("\n-------------------");
        println!("Removing character:");
        remove_character();
        println!("Character removed");
        println!("\n-------------------");
        println!("Saving character:");
        save_character();
        println!("Character saved");
        println!("\n-------------------");
        println!("Retrieving character:");
        get_character();
        println!("Character retrieved");
        println!("\n-------------------");
        println!("Removing character:");
        remove_character();
        println!("Character removed");
    }

    // #[test]
    pub fn save_character() {
        let test_character = get_const_test_character();
        println!("test_character - {:?}", test_character);

        let saved_character = character_controller::save(&test_character);
        println!("saved_character - {:?}", saved_character);

        assert_eq!(test_character, saved_character);
    }

    // #[test]
    pub fn get_character() {
        let test_character = get_const_test_character();
        let name = &test_character.name;
        println!("test_character - {:?}", test_character);

        let returned_character = character_controller::get_by_name(name);

        if returned_character.name == "" {
            println!("Character not found.");
            assert_eq!(returned_character, get_empty_caracter());
        } else {
            println!("returned_character - {:?}", returned_character);
            assert_eq!(test_character, returned_character);
        }
    }

    // #[test]
    pub fn remove_character() {
        let test_character = get_const_test_character();
        println!("test_character - {:?}", test_character);

        character_controller::remove(&test_character);
        let returned_character = character_controller::get_by_name(&test_character.name);
        println!("returned_character - {:?}", returned_character);

        assert_eq!(returned_character, get_empty_caracter());
    }
}
