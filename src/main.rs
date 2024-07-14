#[path = "terminal/term.rs"] mod term;
#[path = "terminal/zinc.rs"] mod zinc;
use zinc::*;
use term::*;

use std::io::{stdout, Result};
use ratatui::{backend::CrosstermBackend, Terminal}; 
use crossterm::event::{self, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    set_up_terminal().expect("Failed to initialize terminal.");
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut state = State::new();

    loop {
        State::run(&mut state, &mut terminal)?;


        
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    restore_terminal().expect("Failed to restore terminal to initial state, to do so manually, type 'reset'.");
    Ok(())
}
