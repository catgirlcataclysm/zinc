use std::io::{self, Stdout};

use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Style, Stylize}, symbols, widgets::{Block, Tabs}, Frame, Terminal};

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
            Constraint::Percentage(10),
            Constraint::Percentage(90),
        ]
        )
        .split(frame.size());

        let tabs = Tabs::new(vec!["Device", "Distro", "Filesystem", "Hostname"])
            .block(Block::bordered())
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(2)
            .divider(symbols::DOT)
            .padding("->", "<-");

        frame.render_widget(tabs, layout[1])
    
    }

    fn handle_events(&self,) -> io::Result<()> {
        todo!()
    }
}