use color_eyre::SectionExt;
use color_eyre::{
    Section,
    eyre::{Result, eyre},
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Clear, List, ListItem, ListState};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Duration;

pub enum CurrentScreen {
    Dashboard,
}

pub enum SelectionLevel {
    Year,
    Day,
}

pub struct App {
    pub exit: bool,
    pub current_screen: CurrentScreen,

    pub show_modal: bool,
    pub selection_level: SelectionLevel,

    pub available_years: Vec<String>,
    pub selected_year_index: usize,
    pub available_days: Vec<String>,
    pub selected_day_index: usize,

    pub current_year: String,
    pub current_day: String,
}

impl App {
    pub fn new() -> Self {
        let init_year = 2025.to_string();
        let init_day = 01.to_string();
        Self {
            exit: false,
            show_modal: false,
            selection_level: SelectionLevel::Year,
            current_screen: CurrentScreen::Dashboard,
            available_years: vec!["2025".to_string(), "2024".to_string(), "2023".to_string()],
            selected_year_index: 0,
            available_days: (1..=25).map(|d| format!("{:02}", d)).collect(),
            selected_day_index: 0,
            current_year: init_year,
            current_day: init_day,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.current_screen {
            CurrentScreen::Dashboard => {
                self.draw_dashboard(frame, &self.current_year, &self.current_day)
            }
        }

        if self.show_modal {
            self.draw_selection_modal(frame);
        }
    }

    fn draw_selection_modal(&self, frame: &mut Frame) {
        let area = self.centered_rect(60, 50, frame.area());
        frame.render_widget(Clear, area);

        let (title, items, selected_index) = match self.selection_level {
            SelectionLevel::Year => (
                " Select Year",
                &self.available_years,
                self.selected_year_index,
            ),
            SelectionLevel::Day => (" Select Day", &self.available_days, self.selected_day_index),
        };

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect();

        let list = List::new(list_items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(Some(selected_index));
        frame.render_stateful_widget(list, area, &mut state);
    }
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    fn draw_dashboard(&self, frame: &mut Frame, year: &String, day: &String) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        let block = Block::default()
            .title(" Advent of Code CLI ")
            .borders(Borders::ALL);

        let text = vec![
            "Welcome to the AOC TUI (Modern Init + Error Handling)!",
            "",
            "Controls:",
            "  'q' -> Quit",
            "  'e' -> Return an Error (Test color-eyre)",
            "  'p' -> Trigger a Panic (Test panic hook)",
            "",
            &format!(
                "You are viewing the dashboard for Year: {}, Day: {}",
                year, day
            ),
        ]
        .join("\n");

        let p = Paragraph::new(text).block(block);
        frame.render_widget(p, chunks[0]);
    }

    fn nav_up(&mut self) {
        match &self.selection_level {
            SelectionLevel::Year => {
                if self.selected_year_index > 0 {
                    self.selected_year_index -= 1;
                }
            }
            SelectionLevel::Day => {
                if self.selected_day_index > 0 {
                    self.selected_day_index -= 1;
                }
            }
        }
    }

    fn nav_down(&mut self) {
        match &self.selection_level {
            SelectionLevel::Year => {
                if self.selected_year_index + 1 < self.available_years.len() {
                    self.selected_year_index += 1;
                }
            }
            SelectionLevel::Day => {
                if self.selected_day_index + 1 < self.available_days.len() {
                    self.selected_day_index += 1;
                }
            }
        }
    }

    fn nav_enter(&mut self) {
        if self.show_modal {
            match &self.selection_level {
                SelectionLevel::Year => {
                    self.current_year = self.available_years[self.selected_year_index].clone();
                    self.selection_level = SelectionLevel::Day;
                }
                SelectionLevel::Day => {
                    self.current_day = self.available_days[self.selected_day_index].clone();
                    self.show_modal = false;
                }
            }
        }
    }

    fn generate_missing_structure() {
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    // CTRL + C
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        self.exit = true;
                    }
                    // ESC
                    KeyCode::Esc => {
                        self.exit = true;
                    }
                    // Up Arrow
                    KeyCode::Up => {
                        self.nav_up();
                    }
                    KeyCode::Char('k') => {
                        self.nav_up();
                    }
                    // Down Arrow
                    KeyCode::Down => {
                        self.nav_down();
                    }
                    KeyCode::Char('j') => {
                        self.nav_down();
                    }
                    // Enter
                    KeyCode::Enter => {
                        self.nav_enter();
                    }

                    // C
                    KeyCode::Char('c') => {
                        self.selection_level = SelectionLevel::Year;
                        self.show_modal = !self.show_modal;
                    }

                    // Case 'e': Standard Error
                    KeyCode::Char('e') => {
                        return Err(eyre!("Simulated Error Triggered!"))
                            .suggestion("Don't press 'e' next time.")
                            .with_section(|| {
                                "This is an extra section explaining why this error is fake."
                                    .to_string()
                                    .header("Context:")
                            });
                    }

                    // Case 'p': Panic
                    KeyCode::Char('p') => {
                        panic!("Simulated Panic! This is a crash.");
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
