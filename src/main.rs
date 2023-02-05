mod pokemon;
mod ui;
mod env;
mod widget;
mod util;
mod args;

use clap::Parser;
use pokemon::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use widget::pmlist::PokemonListStatus;
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
    let args = args::Args::parse();

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

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
    let pm_dex = PokemonListStatus::new(pokemon);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut pm_dex: PokemonListStatus) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut pm_dex))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

                KeyCode::Down => pm_dex.next(),
                KeyCode::Char('j') => pm_dex.next(),

                KeyCode::Up => pm_dex.previous(),
                KeyCode::Char('k') => pm_dex.previous(),

                KeyCode::Left => pm_dex.dex.previous(),
                KeyCode::Char('h') => pm_dex.dex.previous(),

                KeyCode::Right => pm_dex.dex.next(),
                KeyCode::Char('l') => pm_dex.dex.next(),
                _ => {}
            }
        }
    }
}