use color_eyre::Result;
use sqlx::{SqlitePool, prelude::FromRow, sqlite::SqlitePoolOptions};

#[derive(Clone, PartialEq, Eq, Debug, FromRow)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq, Debug, FromRow)]
pub struct Record {
    pub id: i64,
}

pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub async fn new() -> Result<Self> {
        // TODO: read  connection_url from DATABASE_URL env var?
        return Ok(Store {
            pool: SqlitePoolOptions::new()
                .max_connections(2)
                .connect("sqlite://dev.db")
                .await?,
        });
    }

    pub async fn get_locations(&self) -> Result<Vec<Location>> {
        let rows: Vec<Location> = sqlx::query_as!(
            Location,
            r#"
                SELECT
                id AS "id!",
                name AS "name!",
                tag AS "tag!"
                FROM Location
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        return Ok(rows);
    }

    pub async fn delete_location_by_name(&self, name: &str) -> Result<()> {
        let _ = sqlx::query_as!(
            Location,
            r#"
                DELETE from Location
                WHERE name = ?
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_location(&self, name: String, tag: Option<String>) -> Result<Location> {
        let utag = tag.unwrap_or("".to_string());
        let row: Location = sqlx::query_as!(
            Location,
            r#"
                INSERT INTO Location (name, tag)
                VALUES (?, ?)
                RETURNING id as "id!", name as "name!", tag as "tag!"
            "#,
            name,
            utag
        )
        .fetch_one(&self.pool)
        .await?;
        return Ok(row);
    }
}
