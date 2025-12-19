use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

/// Context of app
pub struct Context {
    pub panels: HashMap<String, Box<dyn Panel>>,
    pub focussed: String,
}

pub trait Panel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect);
    fn handle_input(&mut self, event: Event);
    fn is_focussed(&self, event: Event) -> bool;
}

pub struct DebugPanel {}
impl Panel for DebugPanel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {}
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
        focussed: "main".into(),
    };
    context.panels["main".into()] = DebugPanel {};

    loop {
        terminal.draw(render)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    if c == 'q' {
                        break Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
