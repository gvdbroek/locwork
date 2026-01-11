use crossterm::event;
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block,
        calendar::{CalendarEventStore, Monthly},
    },
};
use time::{Date, Duration, Month, OffsetDateTime, Weekday};

use crate::{
    panels::{Action, Panel, record_modal::RecordModalData},
    store::Record,
};

pub struct CalendarPanel {
    pub label: String,
    pub tag: String,
    pub records: Vec<Record>,
    pub selected: Date,
}

impl CalendarPanel {
    pub async fn new(records: Option<Vec<Record>>) -> Self {
        CalendarPanel {
            label: "Calendar".to_string(),
            tag: " ²".to_string(),
            records: records.unwrap_or_default(),
            selected: OffsetDateTime::now_utc().date(),
        }
    }
}

fn bound_date_offset(date: Date, days: i64) -> Date {
    // TODO: move date navigation into a 'bound' function, that can offset the date while
    // staying inside month bound, another hotkey we'll use SHIFT+H and SHIFT+L to navigate months
    let next_date = date.checked_add(Duration::days(days)).unwrap();
    next_date
}
impl Panel for CalendarPanel {
    fn update(&mut self, action: &Action) {
        if let Action::LoadNavigateDateSuccess(records) = action {
            self.records = records.clone();
        }
    }

    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Action> {
        match key_event.code {
            // Move to days
            event::KeyCode::Char('j') => self.selected = bound_date_offset(self.selected, 7),
            event::KeyCode::Char('k') => self.selected = bound_date_offset(self.selected, -7),
            event::KeyCode::Char('h') => self.selected = bound_date_offset(self.selected, -1),
            event::KeyCode::Char('l') => self.selected = bound_date_offset(self.selected, 1),

            // Move Months
            event::KeyCode::Char('H') => {
                let prev_month = self.selected.saturating_sub(Duration::days(31));
                return Some(Action::StartNavigateDate(prev_month));
            }
            event::KeyCode::Char('L') => {
                let next_month = self.selected.saturating_add(Duration::days(31));
                return Some(Action::StartNavigateDate(next_month));
            }

            // Today
            event::KeyCode::Char('t') => self.selected = OffsetDateTime::now_utc().date(),
            event::KeyCode::Char('D') => {}
            event::KeyCode::Char('A') => {
                return Some(Action::AddRecord(RecordModalData {
                    location: "asdasd".to_string(),
                    log_type: crate::store::LogType::Work,
                    date: self.selected,
                }));
            }
            _ => return None,
        }
        Some(Action::Processing)
    }

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, focussed: bool) {
        let label = Span::raw(self.label.clone());
        let tag_style = Style::default().fg(ratatui::style::Color::LightRed);
        let tagspan = Span::raw(&self.tag).style(tag_style.bold());

        let title = Line::raw("").spans([tagspan, label]);
        let mut block = Block::bordered().title(title);
        let block_inner = block.inner(area);

        if focussed {
            block = block.border_style(Style::default().fg(ratatui::style::Color::LightRed));
            // list = list.highlight_style(
            //     Style::new()
            //         .fg(ratatui::style::Color::LightRed)
            //         .add_modifier(Modifier::BOLD),
            // );

            // frame.render_stateful_widget(list, block_inner, &mut self.state);
        } else {
            // list = list.highlight_style(Style::new().add_modifier(Modifier::BOLD));
            // frame.render_stateful_widget(list, block_inner, &mut self.state);
        }

        let today_style = Style::default().add_modifier(Modifier::UNDERLINED);
        let surrounding = Style::default().dark_gray();
        let cal_headers = Style::default().bold();
        let _holiday_style = Style::default().fg(Color::LightMagenta);
        let selected_style = Style::default().bg(Color::Red);
        let weekend_style = Style::default().dark_gray();
        let future_style = Style::default().fg(Color::Rgb(100, 100, 100));

        let mut styles = CalendarEventStore::today(today_style);

        // TODO: convert dates from records into styled dates

        // iterate over month and higlight all days
        let first_day =
            Date::from_calendar_date(self.selected.year(), self.selected.month(), 1).unwrap();
        for i in 0..31 {
            if let Some(current_day) = first_day.checked_add(Duration::days(i)) {
                if [Weekday::Saturday, Weekday::Sunday].contains(&current_day.weekday()) {
                    styles.add(current_day, weekend_style);
                } else if current_day > OffsetDateTime::now_utc().date() {
                    styles.add(current_day, future_style);
                }
            }
        }

        // Selected day
        styles.add(self.selected, selected_style);

        let d = Date::from_calendar_date(2026, Month::January, 1).unwrap();
        let cal = Monthly::new(d, styles)
            .show_weekdays_header(cal_headers)
            .show_month_header(Style::default())
            .show_surrounding(surrounding);

        // let cal = Monthly::new(d, CalendarEventStore::default());

        frame.render_widget(block, area);

        frame.render_widget(cal, block_inner);
    }
}
