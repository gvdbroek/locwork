use color_eyre::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
#[derive(Clone, PartialEq, Eq, Debug)]

pub struct Location {
    pub id: u64,
    pub name: String,
    pub tag: String,
}

pub struct Store {
    connection_url: String,
    pool: SqlitePool,
}

impl Store {
    pub async fn new() -> Result<Self> {
        return Ok(Store {
            connection_url: ":memory:".to_string(),
            pool: SqlitePoolOptions::new()
                .max_connections(2)
                .connect(":memory:")
                .await?,
        });
    }
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
    let location: Location = Location {
        name: name,
        tag: tag.unwrap_or("".to_string()),
    };
    todo!();
}
