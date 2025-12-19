use std::collections::HashMap;

use color_eyre::{Result, owo_colors::OwoColorize};
use crossterm::{
    event::{self, Event},
    style::{self, Color},
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

/// Context of app
pub struct Context {
    pub panels: HashMap<String, Box<dyn Panel>>,
    pub focussed: String,
}

pub trait Panel {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect, focussed: bool);
    fn handle_input(&mut self, event: Event);
    fn is_focussed(&self, event: Event) -> bool;
}

pub struct DebugPanel {
    pub title: String,
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
        focussed: "main".into(),
    };

    let debug_panel = DebugPanel {
        title: "Panel1".to_string(),
    };
    let debug_panel2 = DebugPanel {
        title: "Panel2".to_string(),
    };

    context.panels.insert("main".into(), Box::new(debug_panel));
    context.panels.insert("alt".into(), Box::new(debug_panel2));
    loop {
        terminal.draw(|frame| {
            let row_constraint = (0..2).map(|_| Constraint::Fill(1));
            let horizontal = Layout::vertical(row_constraint).spacing(1);
            let rects = horizontal.split(frame.area());
            let rect = rects.first().unwrap().to_owned();
            let lest = rects.last().unwrap().to_owned();

            // Paragraph::new("Hi?").block(Block::bordered()).render(rect);

            let pan = context.panels.get("main".into());
            match pan {
                Some(panel) => panel.render(frame, rect, context.focussed == "main".to_string()),
                None => (),
            }

            let pan2 = context.panels.get("alt".into());
            match pan2 {
                Some(panel) => panel.render(frame, lest, context.focussed == "alt".to_string()),
                None => (),
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    match c {
                        'q' => break Ok(()),
                        '2' => context.focussed = "alt".to_string(),
                        '1' => context.focussed = "main".to_string(),
                        _ => {}
                    }
                    // if c == 'q' {
                    //     break Ok(());
                    // }
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
