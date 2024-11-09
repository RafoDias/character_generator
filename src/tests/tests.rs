#[cfg(testetest)]
mod tests {
    use super::*;

    #[test]
    pub fn save_character() {
        let character = Character {
            name: "Isabelle".to_string(),
            level: 1,
            class: Class::Rogue,
            birthday: NaiveDate::from_ymd_opt(2004, 04, 21).unwrap(),
        };

        let saved_character = character_controller::save(character);

        assert_eq!(character.name, saved_character.name);
        assert_eq!(character.level, saved_character.level);
        assert_eq!(character.class, saved_character.class);
        assert_eq!(character.birthday, saved_character.birthday);
    }
}
