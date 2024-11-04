use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Class {
    Warrior,
    Cleric,
    Rogue,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Class::Warrior => write!(f, "Warrior"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Rogue => write!(f, "Rogue"),
        }
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Class::Warrior => write!(f, "Warrior"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Rogue => write!(f, "Rogue"),
        }
    }
}

impl From<Class> for i32 {
    fn from(class: Class) -> i32 {
        match class {
            Class::Warrior => 1,
            Class::Cleric => 2,
            Class::Rogue => 3,
        }
    }
}
