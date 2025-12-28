mod panels;
mod store;
use tokio::sync::mpsc::{Sender, channel};

use color_eyre::Result;
use crossterm::event::{self, Event, EventStream};
use futures_util::StreamExt;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    panels::{
        Action,
        debug_panel::DebugPanel,
        location_panel::{LocationsPanel, PanelType},
        modal::{ActiveModal, AddLocationModal, InputModalResult},
        panel::Panel,
    },
    store::{Location, Store},
};

/// Context of app
pub struct Context {
    pub panels: HashMap<PanelType, Box<dyn Panel>>,
    pub rects: HashMap<PanelType, Rect>,
    pub focussed: PanelType,
    pub active_modal: ActiveModal,
}
impl Context {
    pub fn process_action(&mut self, action: Action, tx: Sender<Action>, store: Arc<Store>) {
        match action {
            Action::None => {}
            Action::CancelModal => {
                self.active_modal = ActiveModal::None;
            }
            Action::AddLocation(m) => {
                self.active_modal = ActiveModal::AddLocation(AddLocationModal::new());
            }
            Action::ConfirmAddLocation(name) => {
                self.active_modal = ActiveModal::None;
                let tx_clone = tx.clone();
                let store_clone = Arc::clone(&store);
                tokio::spawn(async move {
                    if let Ok(_) = store_clone.add_location(name, None).await {
                        let locations = store_clone.get_locations().await.unwrap();
                        let _ = tx_clone.send(Action::AddLocationDbSuccess(locations)).await;
                    }
                });
            }
            Action::AddLocationDbSuccess(ref locations) => {
                if let Some(panel) = self.panels.get_mut(&PanelType::Locations) {
                    panel.update(&action);
                }
            }
            Action::DeleteLocation(name) => {}
            Action::Skipped => {}
            Action::Processing => {}
        }
    }
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
    let mut state = Context {
        panels: HashMap::new(),
        rects: HashMap::new(),
        focussed: PanelType::Locations,
        active_modal: ActiveModal::None,
    };
    let store = Arc::new(Store::new().await?);
    let (tsender, mut treceiver) = channel::<Action>(128);
    let mut reader = EventStream::new();

    let debug_panel = DebugPanel {
        title: "Debug panel".to_string(),
    };
    let locations = &store.get_locations().await.unwrap();
    let location_panel = LocationsPanel::new(locations.clone()).await;
    let mut modal = AddLocationModal::new();

    state
        .panels
        .insert(PanelType::Calendar, Box::new(debug_panel));
    state
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
            state.rects.insert(PanelType::Locations, first_rect);
            state.rects.insert(PanelType::Calendar, last_rect);

            // draw
            let visible_panes = vec![PanelType::Locations, PanelType::Calendar];
            for pane_type in &visible_panes {
                if let (Some(panel), Some(rect)) =
                    (state.panels.get_mut(pane_type), state.rects.get(pane_type))
                {
                    // TODO: what is this * nonesense?
                    panel.render(frame, *rect, state.focussed == *pane_type);
                }
            }
            match &state.active_modal {
                ActiveModal::None => {}
                ActiveModal::AddLocation(modal) => modal.render(frame),
            }
        })?;

        // logic
        tokio::select! {
                // Handle User Input (Async)
                maybe_event = reader.next() => {
                    if let Some(Ok(Event::Key(key))) = maybe_event {
                        // 1. Determine Intent (Action)
                        let action: Option<Action> = match &mut state.active_modal {
                            ActiveModal::AddLocation(_m) => {
                                if let Some(res) = _m.handle_input(key)      {
                                    let a: Option<Action> = match res{
                                        InputModalResult::Editting => {None}
                                        InputModalResult::Cancelled => {Some(Action::CancelModal)}
                                        InputModalResult::Confirmed(t) => {Some(Action::ConfirmAddLocation(t))}
                                    };
                                    a
                                }
                                else{
                                    None
                                }
                            },
                            ActiveModal::None => {
                                // Global keys (like quit)
                                match key.code {
                                    event::KeyCode::Char('q') => break Ok(()),
                                    event::KeyCode::Char('1') => { state.focussed = PanelType::Locations; None },
                                    event::KeyCode::Char('2') => { state.focussed = PanelType::Calendar; None },
                                    _ => {
                                        // Pass to panel
                                        state.panels.get_mut(&state.focussed)
                                            .and_then(|panel| panel.handle_input(key))
                                    }
                                }
                            }
                        };

                        // 2. Process Intent
                        if let Some(act) = action {
                            state.process_action(act, tsender.clone(), store.clone());
                        }
                    }
                }
                maybe_bg_action = treceiver.recv() => {
                    if let Some(bg_action) = maybe_bg_action {
            // Process the refresh/success action sent from the tokio::spawn block
            state.process_action(bg_action, tsender.clone(), store.clone());
        }
                }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
