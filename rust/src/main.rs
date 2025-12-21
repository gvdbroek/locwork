mod panels;
mod store;

use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Widget},
};

use crate::{
    panels::{
        debugpanel::DebugPanel,
        locationpanel::{LocationsPanel, PanelType},
        panel::Panel,
    },
    store::Location,
};

/// Context of app
pub struct Context {
    pub panels: HashMap<PanelType, Box<dyn Panel>>,
    pub focussed: PanelType,
}

pub enum Pane {
    Status,
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
        focussed: PanelType::Calendar,
    };

    let debug_panel = DebugPanel {
        title: "Debug panel".to_string(),
    };
    let location_panel = LocationsPanel::new();

    context
        .panels
        .insert(PanelType::Calendar, Box::new(debug_panel));
    context
        .panels
        .insert(PanelType::Locations, Box::new(location_panel));

    loop {
        terminal.draw(|frame| {
            // layout
            let row_constraint = (0..2).map(|_| Constraint::Fill(1));
            let horizontal = Layout::vertical(row_constraint).spacing(1);
            let rects = horizontal.split(frame.area());
            let rect = rects.first().unwrap().to_owned();
            let last = rects.last().unwrap().to_owned();

            // draw
            let locations_pane = context.panels.get(&PanelType::Locations);
            match locations_pane {
                Some(panel) => panel.render(frame, rect, context.focussed == PanelType::Locations),
                None => (),
            }

            let calendar_pane = context.panels.get(&PanelType::Calendar);
            match calendar_pane {
                Some(panel) => panel.render(frame, last, context.focussed == PanelType::Calendar),
                None => (),
            }
        })?;

        // logic

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => match c {
                    'q' => break Ok(()),
                    '2' => context.focussed = PanelType::Calendar,
                    '1' => context.focussed = PanelType::Locations,
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
