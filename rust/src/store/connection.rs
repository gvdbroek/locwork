use crate::store::{Location, LogType, Record};
use color_eyre::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{
    fs::{self},
    path::PathBuf,
};
use time::Date;

pub struct Store {
    pool: SqlitePool,
}
fn get_db_path() -> PathBuf {
    let dirs = directories::ProjectDirs::from("be", "waystone", "locwork").unwrap();
    let mut config_dir = dirs.config_dir().to_path_buf();
    config_dir.push("app.db");
    // makes config dir ~/.config/locwork/app.db
    config_dir
}

impl Store {
    pub async fn new() -> Result<Self> {
        // TODO: read  connection_url from DATABASE_URL env var?
        let parent_folder = get_db_path().parent().unwrap().to_owned();
        if !fs::exists(&parent_folder).unwrap() {
            fs::create_dir_all(parent_folder).unwrap();
        }
        let ops = SqliteConnectOptions::new()
            .filename(get_db_path())
            .create_if_missing(true);

        let store = Store {
            pool: SqlitePool::connect_with(ops).await?,
        };
        sqlx::migrate!("./migrations")
            .run(&store.pool)
            .await
            .unwrap();

        Ok(store)
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

    pub async fn add_record(
        &self,
        date: Date,
        log_type: LogType,
        location: Location,
    ) -> Result<Record> {
        let inserted: Record = sqlx::query_as!(
            Record,
            r#"
            INSERT INTO Record (date, location_id, log_type)
            VALUES (?, ?, ?)
            RETURNING 
                id as "id!",
                date as "date: Date", 
                location_id as "location_id!",
                log_type as "log_type: LogType"
            "#,
            date,
            location.id,
            log_type,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(inserted)
    }
}
