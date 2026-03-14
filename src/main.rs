use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use ratatui::Terminal;

use std::{error::Error, io};


mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Run app in terminal
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Reverse terminal state to normal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    }
    else if let Err(some_error) = res{
        println!("{:?}", some_error);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where 
    io::Error: From<B::Error>,
{

}