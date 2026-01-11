use crate::panels::location_modal::AddLocationModal;
use crate::panels::record_modal::AddRecordModal;

pub enum ActiveModal {
    None,
    AddLocation(AddLocationModal),
    AddRecord(AddRecordModal),
}
