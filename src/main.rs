use ability::Ability;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pokemon::PokemonEntity;
use poketex::pokemon::{AbilityMap, pokemon::PokemonEntity};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::ScrollbarState,
    Terminal,
};
use serde_json::from_str;
use std::{collections::HashMap, error::Error, io, rc::Rc};
use tui_input::Input;
use widget::pmlist::PokemonListStatus;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

    let Ok((pokemon, ability)) = load_data() else {
        println!("load data error");
        std::process::exit(2);
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let pm = PokemonListStatus::new(pokemon);
    let app = AppState::new(pm, ability);
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
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            if handle_key(&mut app, event).is_exit() {
                return Ok(());
            }
        }
    }
}

fn load_data() -> Result<(Vec<PokemonEntity>, AbilityMap), ()> {
    match (
        from_str(include_str!("data/data.json")),
        from_str(include_str!("data/ability.json")),
    ) {
        (Ok(pokemon), Ok(ability)) => Ok((
            pokemon as Vec<PokemonEntity>,
            ability as AbilityMap,
        )),
        _ => Err(()),
    }
}
