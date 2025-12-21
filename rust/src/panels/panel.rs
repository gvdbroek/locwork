use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{Frame, style::Style, text::Span, widgets::Block};

pub trait Panel {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let mut block = Block::bordered();
        let inner = block.inner(area);
        let span = Span::raw("this is a default span");
        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
        }
        frame.render_widget(block, area);
        frame.render_widget(span, inner);
    }
    fn handle_input(&mut self, key: KeyEvent) -> Result<HandleEventResult>;
}
#[derive(Eq, PartialEq)]
pub enum HandleEventResult {
    Processing,
    Skipped,
}

#[derive(Eq, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}
#[derive(Eq, PartialEq)]
pub enum InputModelInputResult {
    Editting,
    Cancelled,
    Confirmed,
}

#[derive(Eq, PartialEq)]
pub struct InputModal {
    pub input: String,
    character_index: usize,
    input_mode: InputMode,
}

impl InputModal {
    // https://ratatui.rs/examples/apps/user_input/
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Editing,
            input: String::new(),
            character_index: 0,
        }
    }
    pub fn clear(&mut self) {
        self.input = String::new();
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> Result<InputModelInputResult> {
        match key_event.code.as_char() {
            Some(c) => {
                self.enter_char(c);
                return Ok(InputModelInputResult::Editting);
            }
            None => (),
        }

        if key_event.code.is_esc() {
            return Ok(InputModelInputResult::Cancelled);
        }
        if key_event.code.is_enter() {
            return Ok(InputModelInputResult::Confirmed);
        }

        // ignore all other special keys
        Ok(InputModelInputResult::Editting)
    }
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
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
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        // self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let input =
            Paragraph::new(self.input.as_str()).block(Block::bordered().title("Add new Location"));
        frame.render_widget(Clear, area);
        frame.render_widget(input, area);
    }
}
