use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, List, ListItem, ListState};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

// Nord color palette
mod colors {
    use ratatui::style::Color;
    pub const FROST_CYAN: Color = Color::Rgb(136, 192, 208);    // #88C0D0
    pub const AURORA_GREEN: Color = Color::Rgb(163, 190, 140);  // #A3BE8C
    pub const AURORA_RED: Color = Color::Rgb(191, 97, 106);     // #BF616A
    pub const AURORA_YELLOW: Color = Color::Rgb(235, 203, 139); // #EBCB8B
    pub const SNOW_WHITE: Color = Color::Rgb(236, 239, 244);    // #ECEFF4
    pub const MUTED_GRAY: Color = Color::Rgb(76, 86, 106);      // #4C566A
}
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub current_year: String,
    pub current_day: String,
    pub selected_year_index: usize,
    pub selected_day_index: usize,
}

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

    pub run_output: Vec<Line<'static>>,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let config = Self::load_config();
        Self {
            exit: false,
            show_modal: false,
            selection_level: SelectionLevel::Year,
            current_screen: CurrentScreen::Dashboard,
            available_years: vec!["2025".to_string(), "2024".to_string(), "2023".to_string()],
            available_days: (1..=25).map(|d| format!("{:02}", d)).collect(),
            run_output: vec![Line::from(Span::styled(
                "Press 'r' to run solution",
                Style::default().fg(colors::MUTED_GRAY),
            ))],
            error_message: None,
            // From config
            current_year: if config.current_year.is_empty() {
                "2025".to_string()
            } else {
                config.current_year
            },
            current_day: if config.current_day.is_empty() {
                "01".to_string()
            } else {
                config.current_day
            },
            selected_year_index: config.selected_year_index,
            selected_day_index: config.selected_day_index,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(terminal).await?;
            self.save_config();
        }
        Ok(())
    }

    fn load_config() -> Config {
        fs::read_to_string("config.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save_config(&self) {
        let config = Config {
            current_year: self.current_year.clone(),
            current_day: self.current_day.clone(),
            selected_year_index: self.selected_year_index,
            selected_day_index: self.selected_day_index,
        };
        let _ = fs::write(
            "config.json",
            serde_json::to_string_pretty(&config).unwrap(),
        );
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.current_screen {
            CurrentScreen::Dashboard => {
                self.draw_dashboard(frame);
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
                " Select Year ",
                &self.available_years,
                self.selected_year_index,
            ),
            SelectionLevel::Day => (" Select Day ", &self.available_days, self.selected_day_index),
        };

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| ListItem::new(item.as_str()).style(Style::default().fg(colors::SNOW_WHITE)))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(Span::styled(title, Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors::FROST_CYAN))
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(colors::AURORA_YELLOW),
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

    fn draw_dashboard(&self, frame: &mut Frame) {
        let size = frame.area();

        // Main vertical layout: Header | Content | Footer
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Footer
            ])
            .split(size);

        self.draw_header(frame, main_chunks[0]);

        // Content: Sidebar | Output
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),  // Sidebar
                Constraint::Percentage(75),  // Output
            ])
            .split(main_chunks[1]);

        self.draw_sidebar(frame, content_chunks[0]);
        self.draw_output(frame, content_chunks[1]);
        self.draw_footer(frame, main_chunks[2]);
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("Advent of Code CLI")
            .style(Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors::FROST_CYAN)));
        frame.render_widget(title, area);
    }

    fn draw_sidebar(&self, frame: &mut Frame, area: Rect) {
        let base = format!("{}/{}", self.current_year, self.current_day);

        // Check file existence for status indicators
        let has_solution = Path::new(&format!("{}/run.rs", base)).exists();
        let has_test = fs::read_to_string(format!("{}/test.txt", base))
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        let has_input = fs::read_to_string(format!("{}/input.txt", base))
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);

        let indicator = |ready: bool| {
            if ready {
                Span::styled("●", Style::default().fg(colors::AURORA_GREEN))
            } else {
                Span::styled("○", Style::default().fg(colors::MUTED_GRAY))
            }
        };

        let lines = vec![
            Line::from(Span::styled(" CONFIG", Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD))),
            Line::from(format!("  Year: {}", self.current_year)),
            Line::from(format!("  Day:  {}", self.current_day)),
            Line::from(""),
            Line::from(Span::styled(" STATUS", Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD))),
            Line::from(vec![Span::raw("  "), indicator(has_solution), Span::raw(" Solution")]),
            Line::from(vec![Span::raw("  "), indicator(has_test), Span::raw(" Test Input")]),
            Line::from(vec![Span::raw("  "), indicator(has_input), Span::raw(" Real Input")]),
            Line::from(""),
            Line::from(Span::styled(" KEYBINDS", Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled("  c  Config", Style::default().fg(colors::SNOW_WHITE))),
            Line::from(Span::styled("  r  Run", Style::default().fg(colors::SNOW_WHITE))),
            Line::from(Span::styled("  q  Quit", Style::default().fg(colors::SNOW_WHITE))),
        ];

        let sidebar = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors::FROST_CYAN)));
        frame.render_widget(sidebar, area);
    }

    fn draw_output(&self, frame: &mut Frame, area: Rect) {
        let output = Paragraph::new(self.run_output.clone())
            .block(
                Block::default()
                    .title(Span::styled(" Output ", Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors::FROST_CYAN))
            )
            .style(Style::default().fg(colors::SNOW_WHITE));
        frame.render_widget(output, area);
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let (content, style) = match &self.error_message {
            Some(err) => (
                format!(" [ERROR] {}", err),
                Style::default().fg(colors::SNOW_WHITE).bg(colors::AURORA_RED),
            ),
            None => (
                String::from(" Ready"),
                Style::default().fg(colors::MUTED_GRAY),
            ),
        };

        let footer = Paragraph::new(content)
            .style(style)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors::FROST_CYAN)));
        frame.render_widget(footer, area);
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
        let base = format!("{}/{}", self.current_year, self.current_day);
        let source_path = format!("{}/run.rs", base);
        let bin_path = format!("/tmp/aoc_runner_{}_{}", self.current_year, self.current_day);

        let mut output_lines: Vec<Line<'static>> = Vec::new();

        // Compile with timing
        let compile_start = Instant::now();
        let compile_cmd = Command::new("rustc")
            .arg(&source_path)
            .arg("-o")
            .arg(&bin_path)
            .output()
            .await;
        let compile_time = compile_start.elapsed();

        match compile_cmd {
            Err(e) => {
                output_lines.push(Line::from(vec![
                    Span::styled("✗ ", Style::default().fg(colors::AURORA_RED)),
                    Span::styled(format!("Failed to start compiler: {}", e), Style::default().fg(colors::AURORA_RED)),
                ]));
                self.run_output = output_lines;
                return Ok(());
            }
            Ok(output) => {
                if !output.status.success() {
                    output_lines.push(Line::from(vec![
                        Span::styled("✗ ", Style::default().fg(colors::AURORA_RED)),
                        Span::styled("Compilation Failed", Style::default().fg(colors::AURORA_RED).add_modifier(Modifier::BOLD)),
                    ]));
                    output_lines.push(Line::from(""));
                    for line in String::from_utf8_lossy(&output.stderr).lines() {
                        output_lines.push(Line::from(Span::styled(
                            line.to_string(),
                            Style::default().fg(colors::AURORA_RED),
                        )));
                    }
                    self.run_output = output_lines;
                    return Ok(());
                }
            }
        }

        // Compilation succeeded
        output_lines.push(Line::from(vec![
            Span::styled("✓ ", Style::default().fg(colors::AURORA_GREEN)),
            Span::styled("Compiled", Style::default().fg(colors::AURORA_GREEN)),
            Span::styled(format!(" ({})", Self::format_duration(compile_time)), Style::default().fg(colors::MUTED_GRAY)),
        ]));
        output_lines.push(Line::from(""));

        // Load expected solutions
        let solution_1 = fs::read_to_string(format!("{}/solution_1.txt", base))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let solution_2 = fs::read_to_string(format!("{}/solution_2.txt", base))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let mut has_any_input = false;

        // Run for each input file
        for input_name in ["test", "input"] {
            let input_path = format!("{}/{}.txt", base, input_name);
            let input_content = match fs::read_to_string(&input_path) {
                Ok(content) if !content.trim().is_empty() => content,
                _ => continue,
            };

            has_any_input = true;

            // Section header
            let header_style = Style::default().fg(colors::FROST_CYAN).add_modifier(Modifier::BOLD);
            let header_text = if input_name == "test" { "TEST" } else { "INPUT" };
            output_lines.push(Line::from(Span::styled(format!("─── {} ───", header_text), header_style)));

            // Execute with timing
            let run_start = Instant::now();
            let run_result = self.execute_binary(&bin_path, &input_content).await;
            let run_time = run_start.elapsed();

            match run_result {
                Err(e) => {
                    output_lines.push(Line::from(vec![
                        Span::styled("  ✗ Error: ", Style::default().fg(colors::AURORA_RED)),
                        Span::raw(e),
                    ]));
                }
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
                        let (status_icon, status_color) = if input_name == "test" {
                            match &solution_1 {
                                Some(exp) if exp == p1 => ("✓", colors::AURORA_GREEN),
                                Some(_) => ("✗", colors::AURORA_RED),
                                None => ("?", colors::AURORA_YELLOW),
                            }
                        } else {
                            ("→", colors::FROST_CYAN)
                        };

                        let mut spans = vec![
                            Span::styled(format!("  {} ", status_icon), Style::default().fg(status_color)),
                            Span::styled("Part 1: ", Style::default().fg(colors::MUTED_GRAY)),
                            Span::styled(p1.clone(), Style::default().fg(colors::SNOW_WHITE).add_modifier(Modifier::BOLD)),
                        ];

                        if input_name == "test" {
                            if let Some(exp) = &solution_1 {
                                if exp != p1 {
                                    spans.push(Span::styled(format!(" (expected: {})", exp), Style::default().fg(colors::AURORA_RED)));
                                }
                            }
                        }

                        output_lines.push(Line::from(spans));
                    }

                    // Format Part 2
                    if let Some(p2) = &part2 {
                        let (status_icon, status_color) = if input_name == "test" {
                            match &solution_2 {
                                Some(exp) if exp == p2 => ("✓", colors::AURORA_GREEN),
                                Some(_) => ("✗", colors::AURORA_RED),
                                None => ("?", colors::AURORA_YELLOW),
                            }
                        } else {
                            ("→", colors::FROST_CYAN)
                        };

                        let mut spans = vec![
                            Span::styled(format!("  {} ", status_icon), Style::default().fg(status_color)),
                            Span::styled("Part 2: ", Style::default().fg(colors::MUTED_GRAY)),
                            Span::styled(p2.clone(), Style::default().fg(colors::SNOW_WHITE).add_modifier(Modifier::BOLD)),
                        ];

                        if input_name == "test" {
                            if let Some(exp) = &solution_2 {
                                if exp != p2 {
                                    spans.push(Span::styled(format!(" (expected: {})", exp), Style::default().fg(colors::AURORA_RED)));
                                }
                            }
                        }

                        output_lines.push(Line::from(spans));
                    }

                    // Timing line
                    output_lines.push(Line::from(Span::styled(
                        format!("  ⏱  {}", Self::format_duration(run_time)),
                        Style::default().fg(colors::MUTED_GRAY),
                    )));

                    // Show stderr, but hide todo!() panics entirely
                    let is_todo_panic = stderr.contains("not yet implemented");
                    if !is_todo_panic && !stderr.trim().is_empty() {
                        output_lines.push(Line::from(vec![
                            Span::styled("  stderr: ", Style::default().fg(colors::AURORA_YELLOW)),
                            Span::raw(stderr.trim().to_string()),
                        ]));
                    }
                }
            }
            output_lines.push(Line::from(""));
        }

        if !has_any_input {
            output_lines.push(Line::from(Span::styled(
                "No inputs provided.",
                Style::default().fg(colors::AURORA_YELLOW),
            )));
            output_lines.push(Line::from(Span::styled(
                "Add content to test.txt or input.txt",
                Style::default().fg(colors::MUTED_GRAY),
            )));
        }

        self.run_output = output_lines;
        Ok(())
    }

    fn format_duration(d: Duration) -> String {
        if d.as_secs() >= 1 {
            format!("{:.2}s", d.as_secs_f64())
        } else if d.as_millis() >= 1 {
            format!("{}ms", d.as_millis())
        } else {
            format!("{}µs", d.as_micros())
        }
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

    async fn handle_events(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Clear error on any key press
                self.error_message = None;

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
                        self.run_output = "Compiling...".to_string();
                        terminal.draw(|frame| self.draw(frame))?;
                        if let Err(e) = self.run_solution().await {
                            self.error_message = Some(e.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
