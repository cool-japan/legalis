//! TUI (Text User Interface) dashboard for Legalis CLI.
//!
//! This module provides:
//! - Full-featured TUI dashboard
//! - Keyboard shortcuts customization
//! - Mouse support for navigation
//! - Split-pane views
//! - Resizable panels

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};
use std::io;

/// Represents the active pane in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    /// Command list pane
    Commands,
    /// Details/output pane
    Details,
    /// Help pane
    Help,
}

/// Keyboard shortcut configuration.
#[derive(Debug, Clone)]
pub struct KeyboardShortcuts {
    /// Key to move up
    pub up: KeyCode,
    /// Key to move down
    pub down: KeyCode,
    /// Key to move left
    pub left: KeyCode,
    /// Key to move right
    pub right: KeyCode,
    /// Key to switch panes
    pub switch_pane: KeyCode,
    /// Key to execute command
    pub execute: KeyCode,
    /// Key to quit
    pub quit: KeyCode,
    /// Key to toggle help
    pub help: KeyCode,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            switch_pane: KeyCode::Tab,
            execute: KeyCode::Enter,
            quit: KeyCode::Char('q'),
            help: KeyCode::Char('?'),
        }
    }
}

impl KeyboardShortcuts {
    /// Create custom keyboard shortcuts.
    pub fn custom() -> Self {
        Self {
            up: KeyCode::Char('k'),
            down: KeyCode::Char('j'),
            left: KeyCode::Char('h'),
            right: KeyCode::Char('l'),
            switch_pane: KeyCode::Tab,
            execute: KeyCode::Enter,
            quit: KeyCode::Char('q'),
            help: KeyCode::Char('?'),
        }
    }
}

/// Panel size configuration.
#[derive(Debug, Clone)]
pub struct PanelSizes {
    /// Left panel percentage (0-100)
    pub left: u16,
    /// Right panel percentage (0-100)
    pub right: u16,
    /// Top panel percentage (0-100)
    pub top: u16,
    /// Bottom panel percentage (0-100)
    pub bottom: u16,
}

impl Default for PanelSizes {
    fn default() -> Self {
        Self {
            left: 30,
            right: 70,
            top: 80,
            bottom: 20,
        }
    }
}

impl PanelSizes {
    /// Resize left panel (adjusts right accordingly).
    pub fn resize_left(&mut self, delta: i16) {
        let new_left = (self.left as i16 + delta).clamp(10, 90);
        self.left = new_left as u16;
        self.right = 100 - self.left;
    }

    /// Resize top panel (adjusts bottom accordingly).
    pub fn resize_top(&mut self, delta: i16) {
        let new_top = (self.top as i16 + delta).clamp(50, 90);
        self.top = new_top as u16;
        self.bottom = 100 - self.top;
    }
}

/// TUI Dashboard application state.
pub struct TuiDashboard {
    /// Available commands
    commands: Vec<String>,
    /// Selected command index
    selected_command: ListState,
    /// Active pane
    active_pane: ActivePane,
    /// Command output/details
    details: Vec<String>,
    /// Help text visibility
    show_help: bool,
    /// Keyboard shortcuts
    shortcuts: KeyboardShortcuts,
    /// Panel sizes
    panel_sizes: PanelSizes,
    /// Mouse support enabled
    mouse_enabled: bool,
    /// Current tab index
    current_tab: usize,
    /// Tab titles
    tabs: Vec<String>,
}

impl Default for TuiDashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiDashboard {
    /// Create a new TUI dashboard.
    pub fn new() -> Self {
        let commands = vec![
            "parse - Parse statute files".to_string(),
            "verify - Verify statute consistency".to_string(),
            "viz - Generate visualizations".to_string(),
            "export - Export to different formats".to_string(),
            "serve - Start API server".to_string(),
            "init - Initialize new project".to_string(),
            "diff - Compare statutes".to_string(),
            "simulate - Run simulations".to_string(),
            "audit - Analyze statute quality".to_string(),
            "format - Format statute files".to_string(),
            "lint - Check statute style".to_string(),
            "test - Test statutes".to_string(),
            "ai - AI-powered features".to_string(),
        ];

        let mut selected_command = ListState::default();
        selected_command.select(Some(0));

        Self {
            commands,
            selected_command,
            active_pane: ActivePane::Commands,
            details: vec![
                "Welcome to Legalis TUI Dashboard!".to_string(),
                "".to_string(),
                "Use arrow keys or hjkl to navigate".to_string(),
                "Press Tab to switch panes".to_string(),
                "Press Enter to execute command".to_string(),
                "Press ? for help".to_string(),
                "Press q to quit".to_string(),
            ],
            show_help: false,
            shortcuts: KeyboardShortcuts::default(),
            panel_sizes: PanelSizes::default(),
            mouse_enabled: true,
            current_tab: 0,
            tabs: vec![
                "Dashboard".to_string(),
                "Commands".to_string(),
                "History".to_string(),
                "Settings".to_string(),
            ],
        }
    }

    /// Enable vim-style keyboard shortcuts.
    pub fn enable_vim_shortcuts(&mut self) {
        self.shortcuts = KeyboardShortcuts::custom();
    }

    /// Toggle mouse support.
    pub fn toggle_mouse(&mut self) {
        self.mouse_enabled = !self.mouse_enabled;
    }

    /// Move selection up.
    fn move_up(&mut self) {
        let i = match self.selected_command.selected() {
            Some(i) => {
                if i == 0 {
                    self.commands.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_command.select(Some(i));
        self.update_details();
    }

    /// Move selection down.
    fn move_down(&mut self) {
        let i = match self.selected_command.selected() {
            Some(i) => {
                if i >= self.commands.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_command.select(Some(i));
        self.update_details();
    }

    /// Switch active pane.
    fn switch_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Commands => ActivePane::Details,
            ActivePane::Details => ActivePane::Help,
            ActivePane::Help => ActivePane::Commands,
        };
    }

    /// Update details pane based on selected command.
    fn update_details(&mut self) {
        if let Some(i) = self.selected_command.selected() {
            if let Some(cmd) = self.commands.get(i) {
                self.details = match cmd.split_whitespace().next().unwrap_or("") {
                    "parse" => vec![
                        "Parse Command".to_string(),
                        "".to_string(),
                        "Parse and validate Legalis statute files.".to_string(),
                        "".to_string(),
                        "Usage: legalis parse <file> [options]".to_string(),
                        "".to_string(),
                        "Options:".to_string(),
                        "  --format <format>  Output format (json, yaml, toml)".to_string(),
                        "  --output <file>    Output file path".to_string(),
                    ],
                    "verify" => vec![
                        "Verify Command".to_string(),
                        "".to_string(),
                        "Verify statute consistency and correctness.".to_string(),
                        "".to_string(),
                        "Usage: legalis verify <file> [options]".to_string(),
                        "".to_string(),
                        "Options:".to_string(),
                        "  --strict          Enable strict validation".to_string(),
                        "  --format <format> Output format".to_string(),
                    ],
                    "viz" => vec![
                        "Visualize Command".to_string(),
                        "".to_string(),
                        "Generate visual representations of statutes.".to_string(),
                        "".to_string(),
                        "Usage: legalis viz <file> -o <output> <format>".to_string(),
                        "".to_string(),
                        "Formats: mermaid, dot, ascii".to_string(),
                    ],
                    "ai" => vec![
                        "AI Features".to_string(),
                        "".to_string(),
                        "AI-powered CLI assistance.".to_string(),
                        "".to_string(),
                        "Subcommands:".to_string(),
                        "  parse   - Parse natural language commands".to_string(),
                        "  intent  - Recognize command intent".to_string(),
                        "  assist  - Get AI-powered help".to_string(),
                        "  suggest - Get command suggestions".to_string(),
                        "  complete - Autocomplete commands".to_string(),
                    ],
                    _ => vec![
                        format!("Command: {}", cmd),
                        "".to_string(),
                        "Press Enter to execute this command".to_string(),
                    ],
                };
            }
        }
    }

    /// Execute selected command.
    fn execute_command(&mut self) {
        if let Some(i) = self.selected_command.selected() {
            if let Some(cmd) = self.commands.get(i) {
                self.details = vec![
                    format!("Executing: {}", cmd),
                    "".to_string(),
                    "This would execute the command in a real implementation.".to_string(),
                    "For now, this is a demo mode.".to_string(),
                ];
            }
        }
    }

    /// Toggle help visibility.
    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Handle keyboard input.
    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) -> bool {
        // Handle Ctrl+C for quit
        if key == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            return true;
        }

        if key == self.shortcuts.quit {
            return true;
        }

        if key == self.shortcuts.help {
            self.toggle_help();
            return false;
        }

        if key == self.shortcuts.switch_pane {
            self.switch_pane();
            return false;
        }

        match self.active_pane {
            ActivePane::Commands => {
                if key == self.shortcuts.up || key == KeyCode::Char('k') {
                    self.move_up();
                } else if key == self.shortcuts.down || key == KeyCode::Char('j') {
                    self.move_down();
                } else if key == self.shortcuts.execute {
                    self.execute_command();
                } else if key == KeyCode::Char('-') {
                    self.panel_sizes.resize_left(-5);
                } else if key == KeyCode::Char('+') || key == KeyCode::Char('=') {
                    self.panel_sizes.resize_left(5);
                }
            }
            ActivePane::Details => {
                if key == KeyCode::Char('[') {
                    self.panel_sizes.resize_top(-5);
                } else if key == KeyCode::Char(']') {
                    self.panel_sizes.resize_top(5);
                }
            }
            ActivePane::Help => {
                // Help pane is read-only
            }
        }

        false
    }

    /// Handle mouse input.
    #[allow(dead_code)]
    fn handle_mouse(&mut self, _x: u16, _y: u16, _kind: MouseEventKind) {
        // Mouse support implementation
        // This can be expanded to handle clicks on different panes
    }

    /// Render the UI.
    fn ui(&mut self, f: &mut Frame) {
        let size = f.area();

        // Create main layout with tabs at top
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(size);

        // Render tabs
        let tab_titles: Vec<Line> = self
            .tabs
            .iter()
            .map(|t| Line::from(Span::styled(t.as_str(), Style::default())))
            .collect();

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Navigation"))
            .select(self.current_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(tabs, chunks[0]);

        // Split the main area into left and right panels
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(self.panel_sizes.left),
                Constraint::Percentage(self.panel_sizes.right),
            ])
            .split(chunks[1]);

        // Render commands list
        let commands_style = if self.active_pane == ActivePane::Commands {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = self
            .commands
            .iter()
            .map(|cmd| ListItem::new(cmd.as_str()))
            .collect();

        let commands_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Commands")
                    .border_style(commands_style),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        f.render_stateful_widget(commands_list, main_chunks[0], &mut self.selected_command);

        // Split right panel into details and help (if shown)
        let right_chunks_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(self.panel_sizes.top),
                Constraint::Percentage(self.panel_sizes.bottom),
            ])
            .split(main_chunks[1]);

        let single_chunk_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(main_chunks[1]);

        let right_chunks = if self.show_help {
            &right_chunks_layout[..]
        } else {
            &single_chunk_layout[..]
        };

        // Render details
        let details_style = if self.active_pane == ActivePane::Details {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let details_text = self.details.join("\n");
        let details = Paragraph::new(details_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Details")
                    .border_style(details_style),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(details, right_chunks[0]);

        // Render help if shown
        if self.show_help {
            let help_style = if self.active_pane == ActivePane::Help {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let help_text = vec![
                "Keyboard Shortcuts:",
                "",
                "Navigation:",
                "  ↑/k     - Move up",
                "  ↓/j     - Move down",
                "  Tab     - Switch pane",
                "",
                "Actions:",
                "  Enter   - Execute command",
                "  ?       - Toggle help",
                "  q       - Quit",
                "",
                "Panel Resize:",
                "  -/+     - Resize left panel",
                "  [/]     - Resize top panel",
                "",
                "Mouse: Click to select (if enabled)",
            ]
            .join("\n");

            let help = Paragraph::new(help_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Help")
                        .border_style(help_style),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(help, right_chunks[1]);
        }

        // Render status bar
        let status_text = format!(
            " Active: {:?} | Mouse: {} | Tab: {} | Panels: {}%/{}% ",
            self.active_pane,
            if self.mouse_enabled { "ON" } else { "OFF" },
            self.current_tab + 1,
            self.panel_sizes.left,
            self.panel_sizes.right
        );

        let status =
            Paragraph::new(status_text).style(Style::default().bg(Color::Blue).fg(Color::White));

        let status_area = Rect {
            x: 0,
            y: size.height.saturating_sub(1),
            width: size.width,
            height: 1,
        };

        f.render_widget(status, status_area);
    }
}

/// Run the TUI dashboard.
pub fn run_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = TuiDashboard::new();

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

/// Run the app event loop.
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut TuiDashboard) -> Result<()> {
    loop {
        terminal.draw(|f| app.ui(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if app.handle_key(key.code, key.modifiers) {
                        return Ok(());
                    }
                }
                Event::Mouse(mouse) => {
                    if app.mouse_enabled {
                        app.handle_mouse(mouse.column, mouse.row, mouse.kind);
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal was resized
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_dashboard_creation() {
        let dashboard = TuiDashboard::new();
        assert_eq!(dashboard.commands.len(), 13);
        assert_eq!(dashboard.active_pane, ActivePane::Commands);
        assert!(!dashboard.show_help);
    }

    #[test]
    fn test_keyboard_shortcuts_default() {
        let shortcuts = KeyboardShortcuts::default();
        assert_eq!(shortcuts.up, KeyCode::Up);
        assert_eq!(shortcuts.down, KeyCode::Down);
        assert_eq!(shortcuts.quit, KeyCode::Char('q'));
    }

    #[test]
    fn test_keyboard_shortcuts_custom() {
        let shortcuts = KeyboardShortcuts::custom();
        assert_eq!(shortcuts.up, KeyCode::Char('k'));
        assert_eq!(shortcuts.down, KeyCode::Char('j'));
    }

    #[test]
    fn test_panel_sizes_default() {
        let sizes = PanelSizes::default();
        assert_eq!(sizes.left, 30);
        assert_eq!(sizes.right, 70);
    }

    #[test]
    fn test_panel_resize() {
        let mut sizes = PanelSizes::default();
        sizes.resize_left(10);
        assert_eq!(sizes.left, 40);
        assert_eq!(sizes.right, 60);

        sizes.resize_left(-20);
        assert_eq!(sizes.left, 20);
        assert_eq!(sizes.right, 80);
    }

    #[test]
    fn test_panel_resize_bounds() {
        let mut sizes = PanelSizes::default();
        sizes.resize_left(100); // Should clamp to max
        assert_eq!(sizes.left, 90);

        sizes.resize_left(-100); // Should clamp to min
        assert_eq!(sizes.left, 10);
    }

    #[test]
    fn test_move_selection() {
        let mut app = TuiDashboard::new();
        app.move_down();
        assert_eq!(app.selected_command.selected(), Some(1));

        app.move_up();
        assert_eq!(app.selected_command.selected(), Some(0));
    }

    #[test]
    fn test_switch_pane() {
        let mut app = TuiDashboard::new();
        assert_eq!(app.active_pane, ActivePane::Commands);

        app.switch_pane();
        assert_eq!(app.active_pane, ActivePane::Details);

        app.switch_pane();
        assert_eq!(app.active_pane, ActivePane::Help);

        app.switch_pane();
        assert_eq!(app.active_pane, ActivePane::Commands);
    }

    #[test]
    fn test_toggle_help() {
        let mut app = TuiDashboard::new();
        assert!(!app.show_help);

        app.toggle_help();
        assert!(app.show_help);

        app.toggle_help();
        assert!(!app.show_help);
    }

    #[test]
    fn test_toggle_mouse() {
        let mut app = TuiDashboard::new();
        assert!(app.mouse_enabled);

        app.toggle_mouse();
        assert!(!app.mouse_enabled);

        app.toggle_mouse();
        assert!(app.mouse_enabled);
    }
}
