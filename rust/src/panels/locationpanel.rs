use crossterm::event::Event;
use ratatui::{
    Frame,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List},
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
}

impl LocationsPanel {
    pub fn new() -> Self {
        let locations = store::get_locations().unwrap();
        LocationsPanel {
            title: "Locations".to_string(),
            locations: locations,
            tag: " ¹".to_string(),
        }
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
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let label = Span::raw("Location");
        let mut tag_style = Style::default().fg(ratatui::style::Color::LightRed);
        let tagspan = Span::raw(&self.tag).style(tag_style.bold());

        let title = Line::raw("").spans([tagspan, label]);
        // let title = Line::raw("Location".to_string());
        let mut block = Block::bordered().title(title);
        let inner = block.inner(area);
        let span = Span::raw("this is a location span");
        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
        }
        // let ls = List::new(self.locations.iter().map(|l| l.name.to_string()).collect());
        let ls = List::new(["hi", "i'm", "a", "test"]);

        frame.render_widget(block, area);
        frame.render_widget(ls, inner);
    }
}
