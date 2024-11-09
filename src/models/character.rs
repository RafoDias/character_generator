use chrono::{self, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub level: i8,
    pub class: super::Class,
    pub birthday: NaiveDate,
}
