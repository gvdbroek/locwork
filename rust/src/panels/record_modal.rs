use crate::store::LogType;
use crossterm::event::KeyEvent;
use time::Date;

#[derive(Eq, PartialEq)]
/// The value provided by the Record Modal
pub struct RecordModalData {
    pub date: Date,
    pub location: String,
    pub log_type: LogType,
}

pub enum RecordModalResult {
    Confirmed(RecordModalData),
}

pub struct AddRecordModal {
    state: RecordModalData,
}

impl AddRecordModal {
    pub fn new(date: Date) -> Self {
        Self {
            state: RecordModalData {
                date: date,
                location: "".to_string(),
                log_type: LogType::Work,
            },
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<RecordModalResult> {
        None
    }
}
