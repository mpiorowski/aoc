use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Modern v0.29+ initialization
    let mut terminal = ratatui::init();

    // Run the app result
    let app_result = App::new().run(&mut terminal).await;

    // Modern restore
    ratatui::restore();

    app_result
}

/// Application state
struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
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
            "Welcome to the AOC TUI (Modern Init)!",
            "",
            "Controls:",
            "  'q' -> Quit",
        ]
        .join("\n");

        let p = Paragraph::new(text).block(block);
        frame.render_widget(p, chunks[0]);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
            }
        }
        Ok(())
    }
}

