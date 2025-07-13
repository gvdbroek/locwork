use std::collections::btree_map::Range;

use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use rand::seq::{IndexedRandom, SliceRandom};
use rand::rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLoc {
    pub date: String,
    pub location: String,
    pub working: bool,
}

fn get_random_loc() -> &'static str {
    const LOCATIONS: &[&str] = &["home", "office"];
    let mut rng = rng();
    LOCATIONS.choose(&mut rng).unwrap()
}
fn get_random_bool() -> bool{
    let mut rng = rng();
    [true, false].choose(&mut rng).unwrap().clone()

}

pub fn generate_mock_record(date: String) -> DateLoc {

    DateLoc {
        date: date,
        location: get_random_loc().into(),
        working: get_random_bool(),
    }
}

pub fn generate_mock_data(count:usize) -> Vec<DateLoc> {
    let mut mock_data: Vec<DateLoc> = vec![];
    for  i in 0..=count {
        mock_data.push(generate_mock_record("2025-01-02".into()));
    }

    mock_data
}
pub fn load_mock_data() -> Vec<DateLoc> {
    let mut mock_data: Vec<DateLoc> = vec![];

    mock_data
}
