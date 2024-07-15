use std::io::{self, Stdout};

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style, Stylize}, symbols, widgets::{Block, Borders, Tabs}, Frame, Terminal};

pub struct State {
    device: String,
    distro: String,
    filesystem: String,
    hostname: String,
    exit: bool,
}

impl State {
    pub fn new() -> State {
        State {
            device: String::from("none"),
            distro: String::from("none"),
            filesystem: String::from("none"),
            hostname: String::from("cadmium"),
            exit: false
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    fn render_frame(&self, frame: &mut Frame) {
        let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(95),
        ]
        )
        .split(frame.size());

        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::White));


        let tabs = Tabs::new(vec!["Device", "Distro", "Filesystem", "Hostname"])
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .divider("|")
            .select(0);

        frame.render_widget(tabs, layout[0])
    
    }

    fn handle_events(&mut self,) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
            }
        }
    Ok(())
    }
}