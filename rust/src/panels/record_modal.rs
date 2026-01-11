use std::io::StdoutLock;

use crate::{
    panels::{list_picker::SimpleListPicker, textfield_component::TextField},
    store::{Location, LogType},
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear, List, ListState, Paragraph},
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
enum Dialog {
    None,
    PickLocation,
}

pub struct AddRecordModal {
    pub state: RecordModalData,
    date_field: TextField,
    locations: Vec<Location>,
    active_dialog: Dialog,
    locations_widget_state: ListState,
}

impl AddRecordModal {
    pub fn new(date: Date, locations: Vec<Location>) -> Self {
        let mut new_instance = Self {
            state: RecordModalData {
                date: date,
                location: "".to_string(),
                log_type: LogType::Work,
            },
            date_field: TextField::new("Record Date".to_string(), Some(date.to_string())),
            locations: locations,
            active_dialog: Dialog::None,
            locations_widget_state: ListState::default(),
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
        let page = Block::bordered().title("Add New Log");
        // let chunks = horizontal.split(page.inner(area));
        let inner_area = page.inner(area);
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(inner_area);

        // render main widget
        frame.render_widget(Clear, area);
        frame.render_widget(page, area);
        // frame.render_widget(input, area);
        // render subcomponents
        let date_area = *chunks.first().unwrap();
        let locations_area = &chunks[1];
        let type_area = &chunks[2];
        self.date_field.render(frame, date_area);

        // locations
        let loc_name = self.locations.first().unwrap().name.clone();
        let location_paragraph =
            Paragraph::new(loc_name).block(Block::bordered().title("Location"));
        frame.render_widget(&location_paragraph, *locations_area);

        // types
        let names = LogType::names().first().unwrap().clone();
        let type_widget = Paragraph::new(names).block(Block::bordered().title("Type".to_string()));
        frame.render_widget(type_widget, *type_area);
        match &self.active_dialog {
            Dialog::None => {}
            Dialog::PickLocation => {
                let loc_names: Vec<String> =
                    self.locations.iter().map(|l| l.name.clone()).collect();
                let list = List::new(loc_names);
                frame.render_widget(Clear, inner_area);
                frame.render_stateful_widget(
                    list,
                    inner_area,
                    &mut self.locations_widget_state.clone(),
                )
            }
        }
    }
}
