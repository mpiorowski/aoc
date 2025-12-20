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
            "  'c' -> Configure Year/Day",
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
        self.run_output = "Compiling...".to_string();

        let base = format!("{}/{}", self.current_year, self.current_day);
        let source_path = format!("{}/run.rs", base);
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
                        "Compilation Error:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return Ok(());
                }
            }
        }

        // Load expected solutions
        let solution_1 = fs::read_to_string(format!("{}/solution_1.txt", base))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let solution_2 = fs::read_to_string(format!("{}/solution_2.txt", base))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let mut results = String::new();

        // Run for each input file
        for input_name in ["test", "input"] {
            let input_path = format!("{}/{}.txt", base, input_name);
            let input_content = match fs::read_to_string(&input_path) {
                Ok(content) if !content.trim().is_empty() => content,
                _ => continue, // skip missing or empty
            };

            let run_result = self.execute_binary(&bin_path, &input_content).await;

            results.push_str(&format!("{}:\n", input_name));

            match run_result {
                Err(e) => results.push_str(&format!("  Error: {}\n", e)),
                Ok((stdout, stderr)) => {
                    // Parse PART1 and PART2 from output
                    let mut part1 = None;
                    let mut part2 = None;
                    for line in stdout.lines() {
                        if let Some(val) = line.strip_prefix("PART1:") {
                            part1 = Some(val.to_string());
                        } else if let Some(val) = line.strip_prefix("PART2:") {
                            part2 = Some(val.to_string());
                        }
                    }

                    // Format Part 1
                    if let Some(p1) = &part1 {
                        let status = if input_name == "test" {
                            match &solution_1 {
                                Some(exp) if exp == p1 => " ✓",
                                Some(exp) => &format!(" ✗ (expected: {})", exp),
                                None => "",
                            }
                        } else {
                            ""
                        };
                        results.push_str(&format!("  Part 1: {}{}\n", p1, status));
                    }

                    // Format Part 2
                    if let Some(p2) = &part2 {
                        let status = if input_name == "test" {
                            match &solution_2 {
                                Some(exp) if exp == p2 => " ✓",
                                Some(exp) => &format!(" ✗ (expected: {})", exp),
                                None => "",
                            }
                        } else {
                            ""
                        };
                        results.push_str(&format!("  Part 2: {}{}\n", p2, status));
                    }

                    // Show stderr, but hide todo!() panics entirely
                    let is_todo_panic = stderr.contains("not yet implemented");
                    if !is_todo_panic && !stderr.trim().is_empty() {
                        results.push_str(&format!("  stderr: {}\n", stderr.trim()));
                    }
                }
            }
            results.push('\n');
        }

        if results.is_empty() {
            results = "No inputs provided.\nAdd content to test.txt or input.txt".to_string();
        }

        self.run_output = results;
        Ok(())
    }

    async fn execute_binary(
        &self,
        bin_path: &str,
        input: &str,
    ) -> std::result::Result<(String, String), String> {
        let mut child = Command::new(bin_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.as_bytes()).await;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Runtime error: {}", e))?;

        Ok((
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    fn generate_missing_structure(&mut self) {
        let base = format!("{}/{}", self.current_year, self.current_day);
        if Path::new(&base).exists() {
            return;
        }
        let _ = fs::create_dir_all(&base);
        let files = ["test.txt", "input.txt", "solution_1.txt", "solution_2.txt"];
        for file in files {
            let _ = File::create(format!("{}/{}", base, file));
        }

        let template = format!(
            r#"// Advent of Code {year} - Day {day}
use std::io::{{self, Read}};
use std::panic::catch_unwind;

fn solve_1(input: &str) -> String {{
    todo!("Part 1")
}}

fn solve_2(input: &str) -> String {{
    todo!("Part 2")
}}

fn main() {{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim().to_string();

    match catch_unwind(|| solve_1(&input)) {{
        Ok(result) => println!("PART1:{{}}", result),
        Err(_) => println!("PART1:--"),
    }}
    match catch_unwind(|| solve_2(&input)) {{
        Ok(result) => println!("PART2:{{}}", result),
        Err(_) => println!("PART2:--"),
    }}
}}
"#,
            year = self.current_year,
            day = self.current_day
        );
        let _ = fs::write(format!("{}/run.rs", base), template);

        let cargo_toml = format!(
            r#"[package]
name = "aoc-{year}-{day}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "run"
path = "run.rs"
"#,
            year = self.current_year,
            day = self.current_day
        );
        let _ = fs::write(format!("{}/Cargo.toml", base), cargo_toml);
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

                    // c - config
                    KeyCode::Char('c') => {
                        self.selection_level = SelectionLevel::Year;
                        self.show_modal = !self.show_modal;
                    }

                    // r - run
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
