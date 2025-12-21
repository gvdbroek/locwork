use color_eyre::Result;
#[derive(Clone, PartialEq, Eq, Debug)]

pub struct Location {
    pub id: u64,
    pub name: String,
    pub tag: String,
}

pub fn get_locations() -> Result<Vec<Location>> {
    let locations: Vec<Location> = vec![
        Location {
            id: 1,
            name: "home".to_string(),
            tag: "home".to_string(),
        },
        Location {
            id: 2,
            name: "office".to_string(),
            tag: "office".to_string(),
        },
        Location {
            id: 3,
            name: "OtherOffice".to_string(),
            tag: "office".to_string(),
        },
    ];
    Ok(locations)
}

pub fn add_location(name: String, tag: Option<String>) -> Result<()> {
    todo!();
}
