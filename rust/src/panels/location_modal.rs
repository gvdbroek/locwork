use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::panels::textfield_component::{TextField, TextFieldResult};

// #[derive(Eq, PartialEq)]
pub struct AddLocationModal {
    pub state: LocationModalState,
    // pub input: String,
    //character_index: usize,
    location_field: TextField,
}

#[derive(Eq, PartialEq, Default)]
/// The value provided by the LocationModal
pub struct LocationModalState {
    pub text: String,
}

#[derive(Eq, PartialEq)]
pub enum InputModalResult {
    Editting,
    Cancelled,
    Confirmed(String),
}

impl AddLocationModal {
    // https://ratatui.rs/examples/apps/user_input/
    pub fn new() -> Self {
        Self {
            state: LocationModalState::default(),
            location_field: TextField::new("Add Location".to_string(), None),
        }
    }
    pub fn clear(&mut self) {
        self.location_field.clear();
        self.state.text.clear();
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<InputModalResult> {
        match self.location_field.handle_input(key_event) {
            TextFieldResult::Editting => {
                self.state.text = self.location_field.state.value.clone();
                Some(InputModalResult::Editting)
            }
            TextFieldResult::Cancelled => Some(InputModalResult::Cancelled),
            TextFieldResult::Confirm(ref state) => {
                self.state.text = state.value.clone();
                Some(InputModalResult::Confirmed(self.state.text.clone()))
            }
        }
    }
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        self.location_field.render(frame, area);
    }
}
