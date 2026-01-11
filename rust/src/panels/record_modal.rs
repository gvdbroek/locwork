use crate::{panels::textfield_component::TextField, store::LogType};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
};
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
    Cancelled,
}

pub struct AddRecordModal {
    pub state: RecordModalData,
    date_field: TextField,
}

impl AddRecordModal {
    pub fn new(date: Date) -> Self {
        let mut new_instance = Self {
            state: RecordModalData {
                date: date,
                location: "".to_string(),
                log_type: LogType::Work,
            },
            date_field: TextField::new("Record Date".to_string(), Some(date.to_string())),
        };
        new_instance.date_field.state.value = date.to_string();
        new_instance
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<RecordModalResult> {
        match key_event.code.as_char() {
            Some(_c) => {}
            None => (),
        };
        if key_event.code.is_esc() {
            return Some(RecordModalResult::Cancelled);
        }
        None
    }
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // let input = Paragraph::new(self.date_field.state.value.as_str())
        //     .block(Block::bordered().title("Add new Record"));

        let page = Block::bordered().title("Add New Log");

        let row_constraint = (0..3).map(|_| Constraint::Fill(1));
        let horizontal = Layout::vertical(row_constraint).spacing(1);
        // let chunks = horizontal.split(page.inner(area));
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(page.inner(area));

        // render main widget
        frame.render_widget(Clear, area);
        frame.render_widget(page, area);
        // frame.render_widget(input, area);
        // render subcomponents
        self.date_field.render(frame, *chunks.first().unwrap());
    }
}
