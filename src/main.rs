use std::{io::{stdout, Result}, thread::sleep};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};
use term::*;
#[path = "terminal/term.rs"] mod term;

fn main() -> Result<()> {
    set_up_terminal().expect("Failed to initialize terminal.");

    loop {

        

        
        
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
