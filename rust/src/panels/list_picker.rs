use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{List, ListState},
};

pub struct SimpleListPicker {
    pub values: Vec<String>,
    pub selected: String,
    list_widget: ListState,
}
pub enum SimpleListPickerResult {
    Editting,
    Confirmed(String),
    Cancelled,
}

impl SimpleListPicker {
    pub fn new(values: Vec<String>, selected: Option<String>) -> Self {
        let first_value = values.first().unwrap();
        Self {
            values: values.clone(),
            selected: selected.unwrap_or(first_value.clone()),
            list_widget: ListState::default(),
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> SimpleListPickerResult {
        if key_event.code.is_esc() {
            return SimpleListPickerResult::Cancelled;
        }
        if key_event.code.is_enter() {
            return SimpleListPickerResult::Confirmed(self.selected.clone());
        }
        match key_event.code {
            KeyCode::Char('k') => {
                self.list_widget.select_previous();
                return SimpleListPickerResult::Editting;
            } // up
            //
            KeyCode::Char('j') => {
                self.list_widget.select_next();
                return SimpleListPickerResult::Editting;
            }
            _ => return SimpleListPickerResult::Editting,
        };
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let list = List::new(self.values.clone())
            .highlight_style(Style::new().reversed())
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.list_widget);
    }
}
