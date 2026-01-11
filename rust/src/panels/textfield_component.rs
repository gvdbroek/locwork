use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Clear, Paragraph},
};

#[derive(Default, Clone)]
pub struct TextFieldState {
    pub value: String,
}
impl TextFieldState {
    pub fn new(initial_value: Option<String>) -> Self {
        initial_value.map(|v| Self { value: v }).unwrap_or_default()
    }
}

pub struct TextField {
    pub state: TextFieldState,
    character_index: usize,
    label: String,
}

pub enum TextFieldResult {
    Confirm(TextFieldState),
    Cancelled,
    Editting,
}

impl TextField {
    // https://ratatui.rs/examples/apps/user_input/
    pub fn new(label: String, initial_value: Option<String>) -> Self {
        Self {
            state: TextFieldState::new(initial_value),
            character_index: 0,
            label: label,
        }
    }
    pub fn clear(&mut self) {
        self.state = TextFieldState::default();
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> TextFieldResult {
        match key_event.code.as_char() {
            Some(c) => {
                self.enter_char(c);
                return TextFieldResult::Editting;
            }
            None => (),
        }

        if key_event.code.is_backspace() {
            self.delete_char();
        }
        if key_event.code.is_esc() {
            return TextFieldResult::Cancelled;
        }
        if key_event.code.is_enter() {
            return TextFieldResult::Confirm(self.state.clone());
        }

        // ignore all other special keys
        TextFieldResult::Editting
    }
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.state.value.insert(index, new_char);
        self.move_cursor_right();
    }
    pub fn byte_index(&self) -> usize {
        self.state
            .value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.state.value.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.state.value.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.state.value.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.state.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.state.value.chars().count())
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.state.value.as_str())
            .block(Block::bordered().title(self.label.clone()));
        frame.render_widget(Clear, area);
        frame.render_widget(input, area);
    }
}
