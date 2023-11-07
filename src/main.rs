use std::{error::Error, io, rc::Rc};

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poketex::{env::DEF_LOCALES, pokemon::{PokemonEntity, AbilityMap, PokemonBundle}, ui::ui, keybinding::handle_key, state::{AppState, PokemonListState}};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use serde_json::from_str;

#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    /// locales [zh, ja, en]
    #[arg(short, long, default_value=&"en")]
    pub locale: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

    let Ok((pokemon, ability)) = load_data() else {
        println!("load data error");
        std::process::exit(2);
    };

    let bundle = PokemonBundle {
        ability: Rc::new(ability),
        pokemon: pokemon.into_iter().map(Rc::new).collect(),
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = AppState {
        pokemon_list: PokemonListState {
            bundle: Rc::new(bundle),
            ..Default::default()
        },
        ..Default::default()
    };
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            if handle_key(&mut app, event).is_exit() {
                return Ok(());
            }
        }
    }
}

fn load_data() -> Result<(Vec<PokemonEntity>, AbilityMap), ()> {
    let pokemon: Vec<PokemonEntity> = from_str(include_str!("data/data.json")).expect("load pokemon data error");
    let ability: AbilityMap = from_str(include_str!("data/ability.json")).expect("load ability data error");
    Ok((pokemon, ability))
}
