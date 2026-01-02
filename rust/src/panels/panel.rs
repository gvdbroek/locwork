use crossterm::event::KeyEvent;
use ratatui::{Frame, style::Style, text::Span, widgets::Block};
use time::Date;

use crate::{
    panels::modal::LocationModalState,
    store::{Location, Record},
};
#[derive(PartialEq, Eq, Hash)]
pub enum PanelType {
    Calendar,
    Status,
    Locations,
}

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
    fn handle_input(&mut self, key: KeyEvent) -> Option<Action>;
    fn update(&mut self, _action: &Action) {}
}
#[derive(Eq, PartialEq)]
pub enum Action {
    None,
    CancelModal,
    AddLocation(LocationModalState),
    ConfirmAddLocation(String),
    DeleteLocation(String),
    AddLocationDbSuccess(Vec<Location>),
    DeleteLocationDbSuccess(Vec<Location>),
    StartNavigateDate(Date),
    LoadNavigateDateSuccess(Vec<Record>),
    Processing,
    Skipped,
}
