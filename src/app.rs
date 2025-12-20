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
use std::fs::{self, File};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub enum CurrentScreen {
    Dashboard,
}

pub enum SelectionLevel {
    Year,
    Day,
}

pub enum InputMode {
    Test1,
    Input1,
    Test2,
    Inoput2,
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

    pub solution_1: String,
    pub solution_2: String,

    pub run_output: String,
}

impl App {
    pub fn new() -> Self {
        let init_year = "2025".to_string();
        let init_day = "01".to_string();
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
            solution_1: "0".to_string(),
            solution_2: "0".to_string(),
            run_output: String::new(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
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
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);

        let block = Block::default()
            .title(" Advent of Code CLI ")
            .borders(Borders::ALL);

        let text = vec![
            "Welcome to the AOC TUI (Modern Init + Error Handling)!",
            "",
            "Controls:",
            "  'q' -> Quit",
            "  'r' -> Run Solution",
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

        let output_block = Block::default().title(" Output ").borders(Borders::ALL);

        let output_p = Paragraph::new(self.run_output.clone()).block(output_block);
        frame.render_widget(output_p, chunks[1]);
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
                    self.generate_missing_structure();
                }
            }
        }
    }

    async fn run_solution(&mut self) -> Result<()> {
        self.run_output = "Running...".to_string();

        let source_path = format!("{}/{}/run.rs", self.current_year, self.current_day);
        let bin_path = format!("/tmp/aoc_runner_{}_{}", self.current_year, self.current_day);

        // Compile
        let compile_cmd = Command::new("rustc")
            .arg(&source_path)
            .arg("-o")
            .arg(&bin_path)
            .output()
            .await;

        match compile_cmd {
            Err(e) => {
                self.run_output = format!("Failed to start compiler: {}", e);
                return Ok(());
            }
            Ok(output) => {
                if !output.status.success() {
                    self.run_output = format!(
                        "Compilation Error: \n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return Ok(());
                }
            }
        }

        // Read input file
        let input_path = format!(
            "{}/{}/test_input_1.txt",
            self.current_year, self.current_day
        );
        let input_content = match fs::read_to_string(&input_path) {
            Ok(content) => content,
            Err(e) => {
                self.run_output = format!("Failed to read input file {}: {}", input_path, e);
                return Ok(());
            }
        };

        // Run with input piped to stdin
        let mut child = match Command::new(&bin_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                self.run_output = format!("Failed to start runner: {}", e);
                return Ok(());
            }
        };

        // Write input to stdin
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input_content.as_bytes()).await;
        }

        // Wait for output
        match child.wait_with_output().await {
            Err(e) => self.run_output = format!("Runtime Error: {}", e),
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.run_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
            }
        }
        Ok(())
    }

    fn generate_missing_structure(&mut self) {
        let base = format!("{}/{}", self.current_year, self.current_day);
        if Path::new(&base).exists() {
            return;
        }
        let _ = fs::create_dir_all(&base);
        let files = [
            "run.rs",
            "test_input_1.txt",
            "test_solution_1.txt",
            "input_1.txt",
            "test_input_2.txt",
            "test_solution_2.txt",
            "input_2.txt",
        ];
        for file in files {
            let _ = File::create(format!("{}/{}", base, file));
        }

        let template = format!(
            r#"// Advent of Code {year} - Day {day}
use std::io::{{self, Read}};

pub fn solve(_input: &str) -> String {{
    todo!("Solve AoC {year} Day {day}")
}}

fn main() {{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{{}}", solve(&input));
}}
"#,
            year = self.current_year,
            day = self.current_day
        );
        let _ = fs::write(format!("{}/run.rs", base), template);
    }

    async fn handle_events(&mut self) -> Result<()> {
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

                    // c
                    KeyCode::Char('c') => {
                        self.selection_level = SelectionLevel::Year;
                        self.show_modal = !self.show_modal;
                    }

                    // r
                    KeyCode::Char('r') => {
                        self.run_solution().await?;
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
