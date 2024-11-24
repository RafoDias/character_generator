#[cfg(test)]
pub mod tests {
    use character_controller::{get_by_name, save};

    use crate::*;

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

    #[test]
    pub fn save_character() {
        let test_character = get_const_test_character();
        println!("test_character - {:?}", test_character);

        let saved_character = character_controller::save(&test_character);
        println!("saved_character - {:?}", saved_character);

        assert_eq!(test_character.level, saved_character.level);
        assert_eq!(test_character.name, saved_character.name);
        assert_eq!(test_character.class, saved_character.class);
        assert_eq!(test_character.birthday, saved_character.birthday);
    }

    #[test]
    pub fn get_character() {
        let test_character = get_const_test_character();
        let name = &test_character.name;
        println!("test_character - {:?}", test_character);

        let returned_character = get_by_name(name);
        println!("returned_character - {:?}", returned_character);

        assert_eq!(test_character.name, returned_character.name);
    }

    #[test]
    pub fn remove_character() {
        let test_character = get_const_test_character();
        println!("test_character - {:?}", test_character);

        character_controller::remove(&test_character);
        let returned_character = character_controller::get_by_name(&test_character.name);
        println!("returned_character - {:?}", returned_character);

        assert_eq!(
            returned_character,
            Character {
                name: "".to_string(),
                level: 0,
                class: Class::NoClass,
                birthday: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
            }
        );
    }
}
