use color_eyre::SectionExt;
use color_eyre::{
    Section,
    eyre::{Result, eyre},
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Duration;

pub enum CurrentScreen {
    YearSelection,
    DaySelection { year: String },
    Dashboard { year: String, day: String },
}

pub struct App {
    pub exit: bool,
    pub current_screen: CurrentScreen,

    pub available_years: Vec<String>,
    pub selected_year_index: usize,
    pub available_days: Vec<String>,
    pub selected_day_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            current_screen: CurrentScreen::YearSelection,
            available_years: vec!["2023".to_string(), "2024".to_string(), "2025".to_string()],
            selected_year_index: 0,
            available_days: (1..=31).map(|d| format!("{:02}", d)).collect(),
            selected_day_index: 0,
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
            CurrentScreen::YearSelection => self.draw_year_selection(frame),
            CurrentScreen::DaySelection { year } => self.draw_day_selection(frame, year),
            CurrentScreen::Dashboard { year, day } => self.draw_dashboard(frame),
        }
    }

    fn draw_year_selection(&self, frame: &mut Frame) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .margin(0)
            .split(size);

        // 1. Title
        let title_block = Block::default()
            .borders(Borders::ALL)
            .title(" AoC Manager ");
        let title_text = Paragraph::new("Select a Year to work on")
            .block(title_block)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title_text, chunks[0]);

        // 2. Year List
        let items: Vec<ListItem> = self
            .available_years
            .iter()
            .map(|year| ListItem::new(year.as_str()))
            .collect();

        // Highlight the selected item
        let list = List::new(items)
            .block(Block::default().title(" Years ").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
            .highlight_symbol(">> ");

        // Create a temporary state to render the selection
        // (In a real app with scrolling, we might persist this state in struct App)
        let mut state = ListState::default();
        state.select(Some(self.selected_year_index));
        frame.render_stateful_widget(list, chunks[1], &mut state);
    }

    fn draw_day_selection(&self, frame: &mut Frame, year: &String) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .margin(0)
            .split(size);

        // 1. Title
        let title_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" AoC Manager - Year {} ", year));
        let title_text = Paragraph::new("Select a Day to work on")
            .block(title_block)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title_text, chunks[0]);
    }

    fn draw_dashboard(&self, frame: &mut Frame) {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
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
        ]
        .join("\n");

        let p = Paragraph::new(text).block(block);
        frame.render_widget(p, chunks[0]);
    }

    fn nav_up (&mut self) {
        match &self.current_screen {
            CurrentScreen::YearSelection => {
                if self.selected_year_index > 0 {
                    self.selected_year_index -= 1;
                }
            }
            CurrentScreen::DaySelection { .. } => {
                if self.selected_day_index > 0 {
                    self.selected_day_index -= 1;
                }
            }
            _ => {}
        }
    }

    fn nav_down (&mut self) {
        match &self.current_screen {
            CurrentScreen::YearSelection => {
                if self.selected_year_index + 1 < self.available_years.len() {
                    self.selected_year_index += 1;
                }
            }
            CurrentScreen::DaySelection { .. } => {
                if self.selected_day_index + 1 < self.available_days.len() {
                    self.selected_day_index += 1;
                }
            }
            _ => {}
        }
    }

    fn nav_enter(&mut self) {
        match &self.current_screen {
            CurrentScreen::YearSelection => {
                let selected_year = self.available_years[self.selected_year_index].clone();
                self.current_screen = CurrentScreen::DaySelection { year: selected_year };
            }
            CurrentScreen::DaySelection { year } => {
                let selected_day = self.available_days[self.selected_day_index].clone();
                self.current_screen = CurrentScreen::Dashboard {
                    year: year.clone(),
                    day: selected_day,
                };
            }
            _ => {}
        }
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
