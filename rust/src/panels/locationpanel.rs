use crossterm::event::Event;
use ratatui::{
    Frame,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListState},
};

use crate::{
    panels::{HandleEventResult, panel::Panel},
    store::{self, Location},
};

#[derive(PartialEq, Eq, Hash)]
pub enum PanelType {
    Calendar,
    Status,
    Locations,
}

pub struct LocationsPanel {
    pub title: String,
    pub locations: Vec<Location>,
    pub tag: String,
    state: ListState,
}

impl LocationsPanel {
    pub fn new() -> Self {
        let locations = store::get_locations().unwrap();
        let mut base = LocationsPanel {
            title: "Locations".to_string(),
            locations: locations,
            tag: " ¹".to_string(),
            state: ListState::default(),
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
    fn handle_input(&mut self, event: Event) -> HandleEventResult {
        HandleEventResult::Skipped
    }
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let label = Span::raw("Location");
        let mut tag_style = Style::default().fg(ratatui::style::Color::LightRed);
        let tagspan = Span::raw(&self.tag).style(tag_style.bold());

        let title = Line::raw("").spans([tagspan, label]);
        // let title = Line::raw("Location".to_string());
        let mut block = Block::bordered().title(title);
        let inner = block.inner(area);
        let span = Span::raw("this is a location span");

        let items: Vec<String> = self.locations.iter().map(|l| l.name.clone()).collect();
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .highlight_style(Style::new().reversed())
            // .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
            frame.render_stateful_widget(list, area, &mut self.state);
        } else {
            frame.render_widget(list, area);
        }
        // let ls = List::new(self.locations.iter().map(|l| l.name.to_string()).collect());
        // let ls =
        //     List::new(options).highlight_style(Style::default().fg(ratatui::style::Color::Yellow));

        // let mut state = ListState::default();
        // let items = ["Item 1", "Item 2", "Item 3"];

        frame.render_widget(block, area);
        // frame.render_widget(ls, inner);
    }
}
