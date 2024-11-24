use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Class {
    NoClass,
    Warrior,
    Cleric,
    Rogue,
}

impl Class {
    pub fn iter() -> ClassIter {
        ClassIter {
            current: Class::NoClass,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Class::NoClass => write!(f, "No Class"),
            Class::Warrior => write!(f, "Warrior"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Rogue => write!(f, "Rogue"),
        }
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Class::NoClass => write!(f, "No Class"),
            Class::Warrior => write!(f, "Warrior"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Rogue => write!(f, "Rogue"),
        }
    }
}

impl From<Class> for i32 {
    fn from(class: Class) -> i32 {
        match class {
            Class::NoClass => 0,
            Class::Warrior => 1,
            Class::Cleric => 2,
            Class::Rogue => 3,
        }
    }
}

impl From<i32> for Class {
    fn from(value: i32) -> Class {
        match value {
            0 => Class::NoClass,
            1 => Class::Warrior,
            2 => Class::Cleric,
            3 => Class::Rogue,
            _ => Class::NoClass,
        }
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Class::NoClass, Class::NoClass) => true,
            (Class::Warrior, Class::Warrior) => true,
            (Class::Cleric, Class::Cleric) => true,
            (Class::Rogue, Class::Rogue) => true,
            _ => false,
        }
    }
}

impl Clone for Class {
    fn clone(&self) -> Self {
        match self {
            Class::NoClass => Class::NoClass,
            Class::Warrior => Class::Warrior,
            Class::Cleric => Class::Cleric,
            Class::Rogue => Class::Rogue,
        }
    }
}

impl Copy for Class {}

pub struct ClassIter {
    current: Class,
}

impl Iterator for ClassIter {
    type Item = Class;

    fn next(&mut self) -> Option<Self::Item> {
        let class = match self.current {
            Class::NoClass => Some(Class::Warrior),
            Class::Warrior => Some(Class::Cleric),
            Class::Cleric => Some(Class::Rogue),
            Class::Rogue => None,
        };

        if let Some(next_class) = class {
            self.current = next_class;
        }
        class
    }
}
