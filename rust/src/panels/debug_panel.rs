use crossterm::event::KeyEvent;
use ratatui::{Frame, style::Style, text::Span, widgets::Block};

use crate::panels::{Action, Panel};

pub struct DebugPanel {
    pub title: String,
}

impl Panel for DebugPanel {
    fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let mut block = Block::bordered().title(self.title.as_str());
        let inner = block.inner(area);
        let span = Span::raw("this is a span");
        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
        }
        frame.render_widget(block, area);
        frame.render_widget(span, inner);
    }
    fn handle_input(&mut self, _key: KeyEvent) -> Option<Action> {
        None
    }
}
