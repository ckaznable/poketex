mod pokemon;
mod ui;
mod env;

use ui::{ui, pokemon::PokemonDex};
use pokemon::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use serde_json::from_str;

fn get_pokemon_data() -> Result<Vec<Pokemon>, serde_json::Error> {
    let contents = include_str!("data.json");
    let pokemon: Result<Vec<Pokemon>, serde_json::Error> = from_str(&contents);
    pokemon
}

fn main() -> Result<(), Box<dyn Error>> {
    let pokemon = match get_pokemon_data() {
        Ok(r) => r,
        Err(_) => {
            print!("data error");
            std::process::exit(2);
        },
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let pm_dex = PokemonDex::new(pokemon);
    let res = run_app(&mut terminal, pm_dex);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut pm_dex: PokemonDex) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut pm_dex))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => pm_dex.next(),
                KeyCode::Up => pm_dex.previous(),
                _ => {}
            }
        }
    }
}