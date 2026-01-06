use time::Date;

use crate::store::LogType;

#[derive(Eq, PartialEq)]
/// The value provided by the Record Modal
pub struct RecordModalResult {
    pub date: Date,
    pub location: String,
    pub log_type: LogType,
}
