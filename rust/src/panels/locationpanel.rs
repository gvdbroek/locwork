use color_eyre::eyre::{Error, Result};
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{
    Frame,
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListState},
};

use crate::{
    panels::{
        HandleEventResult,
        panel::{InputModal, InputMode, InputModelInputResult, Panel},
    },
    store::{self, Location},
};

#[derive(PartialEq, Eq, Hash)]
pub enum PanelType {
    Calendar,
    Status,
    Locations,
}

pub struct LocationsPanel {
    pub label: String,
    pub locations: Vec<Location>,
    pub tag: String,
    state: ListState,
    input_mode: InputMode,
    input_modal: InputModal,
}

impl LocationsPanel {
    pub fn new() -> Self {
        let locations = store::get_locations().unwrap();
        let mut base = LocationsPanel {
            label: "Locations".to_string(),
            locations: locations,
            tag: " ¹".to_string(),
            state: ListState::default(),
            input_mode: InputMode::Normal,
            input_modal: InputModal::new(),
        };
        base.state.select_first();
        base
    }

    pub fn reload(&mut self) -> Result<(), ()> {
        self.locations = store::get_locations().unwrap();
        Ok(())
    }
}

impl Panel for LocationsPanel {
    fn handle_input(&mut self, key_event: KeyEvent) -> Result<HandleEventResult> {
        match self.input_mode {
            InputMode::Editing => match self.input_modal.handle_input(key_event) {
                Ok(k) => match k {
                    InputModelInputResult::Editting => {}
                    InputModelInputResult::Cancelled => {
                        self.input_mode = InputMode::Normal;
                        self.input_modal.clear();
                    }
                    InputModelInputResult::Confirmed => {
                        self.input_mode = InputMode::Normal;
                        self.input_modal.clear();
                        store::add_location(self.input_modal.input.clone(), None)?;
                        return Ok(HandleEventResult::Skipped);
                    }
                },
                Err(_) => panic!("Input Modal failed during input"),
            },
            InputMode::Normal => match key_event.code {
                event::KeyCode::Char('j') => self.state.select_next(),
                event::KeyCode::Char('k') => self.state.select_previous(),
                event::KeyCode::Char('a') => {
                    self.input_modal.clear();
                    self.input_mode = InputMode::Editing;
                }
                _ => return Ok(HandleEventResult::Skipped),
            },
        }

        Ok(HandleEventResult::Processing)
    }

    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let label = Span::raw(self.label.clone());
        let tag_style = Style::default().fg(ratatui::style::Color::LightRed);
        let tagspan = Span::raw(&self.tag).style(tag_style.bold());

        let title = Line::raw("").spans([tagspan, label]);
        let mut block = Block::bordered().title(title);
        let block_inner = block.inner(area);

        let items: Vec<String> = self.locations.iter().map(|l| l.name.clone()).collect();
        let mut list = List::new(items)
            .highlight_style(Style::new().reversed())
            .repeat_highlight_symbol(true);

        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
            list = list.highlight_style(
                Style::new()
                    .fg(ratatui::style::Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            );

            frame.render_stateful_widget(list, block_inner, &mut self.state);
        } else {
            list = list.highlight_style(Style::new().add_modifier(Modifier::BOLD));
            frame.render_stateful_widget(list, block_inner, &mut self.state);
        }

        frame.render_widget(block, area);
        // draw on top
        if self.input_mode == InputMode::Editing {
            self.input_modal.render(frame);
        }
    }
}
