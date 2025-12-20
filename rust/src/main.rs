mod store;

use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Widget},
};

use crate::store::Location;

#[derive(PartialEq, Eq, Hash)]
pub enum Panels {
    Calendar,
    Status,
    Locations,
}

/// Context of app
pub struct Context {
    pub panels: HashMap<Panels, Box<dyn Panel>>,
    pub focussed: Panels,
}

pub trait Panel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool);
    fn handle_input(&mut self, event: Event);
    fn is_focussed(&self, event: Event) -> bool;
}
pub enum Pane {
    Status,
}

pub struct DebugPanel {
    pub title: String,
}
pub struct LocationsPanel {
    pub title: String,
    pub locations: Vec<Location>,
}
impl LocationsPanel {
    pub fn new() -> Self {
        let locations = store::get_locations().unwrap();
        LocationsPanel {
            title: "Locations".to_string(),
            locations: locations,
        }
    }

    pub fn reload(&mut self) -> Result<(), ()> {
        self.locations = store::get_locations().unwrap();
        Ok(())
    }
}
impl Panel for DebugPanel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let mut block = Block::bordered().title(self.title.as_str());
        let inner = block.inner(area);
        let span = Span::raw("this is a span");
        if (focussed) {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
        }
        frame.render_widget(block, area);
        frame.render_widget(span, inner);
    }
    fn handle_input(&mut self, event: Event) {}
    fn is_focussed(&self, event: Event) -> bool {
        true
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    // init context and panels
    let mut context = Context {
        panels: HashMap::new(),
        focussed: Panels::Calendar,
    };

    let debug_panel = DebugPanel {
        title: "calendar".to_string(),
    };
    let debug_panel2 = DebugPanel {
        title: "location".to_string(),
    };

    context
        .panels
        .insert(Panels::Calendar, Box::new(debug_panel));
    context
        .panels
        .insert(Panels::Locations, Box::new(debug_panel2));

    loop {
        terminal.draw(|frame| {
            let row_constraint = (0..2).map(|_| Constraint::Fill(1));
            let horizontal = Layout::vertical(row_constraint).spacing(1);
            let rects = horizontal.split(frame.area());
            let rect = rects.first().unwrap().to_owned();
            let last = rects.last().unwrap().to_owned();

            // Paragraph::new("Hi?").block(Block::bordered()).render(rect);

            let locations_pane = context.panels.get(&Panels::Locations);
            match locations_pane {
                Some(panel) => panel.render(frame, rect, context.focussed == Panels::Locations),
                None => (),
            }

            let calendar_pane = context.panels.get(&Panels::Calendar);
            match calendar_pane {
                Some(panel) => panel.render(frame, last, context.focussed == Panels::Calendar),
                None => (),
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => match c {
                    'q' => break Ok(()),
                    '2' => context.focussed = Panels::Calendar,
                    '1' => context.focussed = Panels::Locations,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
