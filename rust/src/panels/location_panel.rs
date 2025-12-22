use std::{clone, sync::Arc};

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
        HandleEventResult,
        panel::{InputModal, InputMode, InputModelInputResult, Panel},
    },
    store::{self, Location, Store},
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
    store: Arc<Store>,
}

impl LocationsPanel {
    pub async fn new(store: Arc<Store>) -> Self {
        let locations = store.get_locations().await.unwrap();
        let mut base = LocationsPanel {
            label: "Locations".to_string(),
            locations: locations,
            tag: " ¹".to_string(),
            state: ListState::default(),
            input_mode: InputMode::Normal,
            input_modal: InputModal::new(),
            store: store,
        };
        base.state.select_first();
        base
    }

    pub async fn reload(&mut self) -> Result<(), ()> {
        // let store_clone = Arc::clone(&self.store);
        self.locations = self.store.get_locations().await.unwrap();
        // self.locations = store_clone.get_locations().unwrap();
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
                        let store_clone = Arc::clone(&self.store);
                        let v = self.input_modal.input.clone();
                        if v.len() == 0 {
                            panic!();
                        }
                        tokio::spawn(async move {
                            let _ = store_clone.add_location(v, None).await;
                        });
                        self.locations.push(Location {
                            id: 0,
                            name: self.input_modal.input.clone(),
                            tag: "".to_string(),
                        });
                        self.input_modal.clear();

                        return Ok(HandleEventResult::Skipped);
                    }
                },
                Err(_) => panic!("Input Modal failed during input"),
            },
            InputMode::Normal => match key_event.code {
                event::KeyCode::Char('j') => self.state.select_next(),
                event::KeyCode::Char('k') => self.state.select_previous(),
                event::KeyCode::Char('d') => {
                    let selected_location_id = self.state.selected().unwrap();
                    let element = self.locations.remove(selected_location_id);
                    let store_clone = Arc::clone(&self.store);
                    tokio::spawn(async move {
                        store_clone
                            .delete_location_by_name(element.name.as_str())
                            .await
                    });
                }
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
