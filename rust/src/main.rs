mod panels;
mod store;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    panels::{
        HandleEventResult,
        debug_panel::DebugPanel,
        location_panel::{LocationsPanel, PanelType},
        panel::Panel,
    },
    store::Store,
};

/// Context of app
pub struct Context {
    pub panels: HashMap<PanelType, Box<dyn Panel>>,
    pub rects: HashMap<PanelType, Rect>,
    pub focussed: PanelType,
}

pub enum Pane {
    Status,
    Location,
    Calendar,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal).await;
    ratatui::restore();
    result
}

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    // init context and panels
    let mut context = Context {
        panels: HashMap::new(),
        rects: HashMap::new(),
        focussed: PanelType::Locations,
    };
    let store = Arc::new(Store::new().await?);

    let debug_panel = DebugPanel {
        title: "Debug panel".to_string(),
    };
    let location_panel = LocationsPanel::new(store).await;

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
            let first_rect = rects.first().unwrap().to_owned();
            let last_rect = rects.last().unwrap().to_owned();

            // let mut pane_rects: HashMap<PanelType, Rect> = HashMap::new();
            context.rects.insert(PanelType::Locations, first_rect);
            context.rects.insert(PanelType::Calendar, last_rect);

            // draw
            let visible_panes = vec![PanelType::Locations, PanelType::Calendar];
            visible_panes.iter().for_each(|pane_type| {
                let pane = context.panels.get_mut(pane_type);
                let rect = context.rects.get(pane_type).unwrap();
                match pane {
                    Some(panel) => {
                        panel.render(frame, rect.clone(), &context.focussed == pane_type);
                    }
                    None => (),
                }
            });
        })?;

        // logic
        let active_panel = context.panels.get_mut(&context.focussed).unwrap();
        let mut active_panel_result: HandleEventResult = HandleEventResult::Skipped;
        if active_panel_result == HandleEventResult::Skipped {
            if let Event::Key(key) = event::read()? {
                active_panel_result = active_panel.handle_input(key).unwrap();
                match active_panel_result {
                    HandleEventResult::Processing => {}
                    HandleEventResult::Skipped => match key.code {
                        event::KeyCode::Char(c) => match c {
                            'q' => break Ok(()),
                            '2' => context.focussed = PanelType::Calendar,
                            '1' => context.focussed = PanelType::Locations,
                            _ => {}
                        },
                        _ => {}
                    },
                }
            }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
