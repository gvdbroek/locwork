use sqlx::prelude::FromRow;
use time::Date;

#[derive(Clone, PartialEq, Eq, Debug, FromRow)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub tag: String,
}

#[derive(Clone, PartialEq, Eq, Debug, FromRow)]
pub struct Record {
    pub id: i64,
    pub location_id: i64,
    pub date: Date,
    pub log_type: LogType,
}

#[derive(Clone, PartialEq, Eq, Debug, sqlx::Type)]
#[repr(i64)]
pub enum LogType {
    Unknown = 0,
    Work = 1,
    Holiday = 2,
    Vacation = 3,
    Sick = 4,
}

// impl From<i64> for LogType {}
