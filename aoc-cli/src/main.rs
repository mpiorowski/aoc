use color_eyre::SectionExt;
use color_eyre::{
    eyre::{eyre, Result},
    Section,
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup color-eyre
    color_eyre::install()?;

    // 2. Setup TUI Panic Hook
    // Ensure we restore the terminal if the app panics
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        original_hook(panic_info);
    }));

    // 3. Initialize Terminal (Modern Pattern)
    let mut terminal = ratatui::init();

    // 4. Run the app
    let app_result = App::new().run(&mut terminal).await;

    // 5. Restore Terminal (Normal Exit)
    ratatui::restore();

    // 6. Print errors if any (from 'e' key)
    app_result
}

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
            "Welcome to the AOC TUI (Modern Init + Error Handling)!",
            "",
            "Controls:",
            "  'q' -> Quit",
            "  'e' -> Return an Error (Test color-eyre)",
            "  'p' -> Trigger a Panic (Test panic hook)",
        ].join("\n");

        let p = Paragraph::new(text).block(block);
        frame.render_widget(p, chunks[0]);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    
                    // Case 'e': Standard Error
                    KeyCode::Char('e') => {
                        return Err(eyre!("Simulated Error Triggered!"))
                            .suggestion("Don't press 'e' next time.")
                            .with_section(|| "This is an extra section explaining why this error is fake.".to_string().header("Context:"));
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
