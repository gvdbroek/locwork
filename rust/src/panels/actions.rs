#[derive(Eq, PartialEq)]
pub enum Action {
    CreateLocation,
    DeleteLocation,
    SetLog,
    DeleteLog,
    SetViewDateRange,
}
