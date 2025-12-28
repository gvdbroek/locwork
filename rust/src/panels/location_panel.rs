use std::{
    clone,
    sync::{Arc, mpsc::Sender},
};

use color_eyre::eyre::Result;
use crossterm::event::{self, KeyEvent};
use ratatui::{
    Frame,
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListState},
};

use crate::{
    panels::{
        Action,
        modal::{AddLocationModal, InputModalResult, InputMode},
        panel::Panel,
    },
    store::{Location, Store},
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
}

impl LocationsPanel {
    pub async fn new(locations: Vec<Location>) -> Self {
        let mut base = LocationsPanel {
            label: "Locations".to_string(),
            locations: locations,
            tag: " ¹".to_string(),
            state: ListState::default(),
            input_mode: InputMode::Normal,
        };
        base.state.select_first();
        base
    }

    pub async fn reload(&mut self) -> Result<(), ()> {
        // let store_clone = Arc::clone(&self.store);
        // self.locations = self.store.get_locations().await.unwrap();
        Ok(())
    }
}

impl Panel for LocationsPanel {
    fn handle_input(&mut self, key_event: KeyEvent) -> Option<Action> {
        match key_event.code {
            event::KeyCode::Char('j') => self.state.select_next(),
            event::KeyCode::Char('k') => self.state.select_previous(),
            event::KeyCode::Char('D') => {
                let selected_location_id = self.state.selected().unwrap();
                let element = self.locations.remove(selected_location_id);
                return Some(Action::DeleteLocation(element.name));
            }
            event::KeyCode::Char('A') => {
                return Some(Action::AddLocation(
                    crate::panels::modal::LocationModalState::default(),
                ));
            }
            _ => return None,
        }
        Some(Action::Processing)
    }

    fn update(&mut self, action: &Action) {
        if let Action::AddLocationDbSuccess(locations) = action {
            self.locations = locations.clone();
        }
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
        // // draw on top
        // if self.input_mode == InputMode::Editing {
        //     self.input_modal.render(frame);
        // }
    }
}
