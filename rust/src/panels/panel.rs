use crossterm::event::Event;
use ratatui::{Frame, style::Style, text::Span, widgets::Block};

pub trait Panel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool) {
        let mut block = Block::bordered();
        let inner = block.inner(area);
        let span = Span::raw("this is a default span");
        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
        }
        frame.render_widget(block, area);
        frame.render_widget(span, inner);
    }
    fn handle_input(&mut self, event: Event) -> HandleEventResult;
}
pub enum HandleEventResult {
    Processing,
    Skipped,
}
