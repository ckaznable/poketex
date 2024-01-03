use std::{error::Error, io, rc::Rc, path::Path};

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poketex::{
    env::DEF_LOCALES,
    keybinding::handle_key,
    pokemon::{AbilityMap, PokemonBundle, PokemonEntity},
    state::{AppState, PokemonListState},
    ui::ui,
};
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

struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    fn init() -> Result<Self, Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let execute_dir = std::env::current_exe()?;
    let execute_dir = execute_dir.parent().ok_or("can't get execute dir path")?;
    let current_dir = std::env::current_dir()?;

    let assets_path = Path::new("colorscripts/small/regular");
    let assets_dir = execute_dir.join(assets_path);
    let assets_dir = if assets_dir.exists() {
        assets_dir
    } else {
        current_dir.join(assets_path)
    };

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
    let mut tui = Tui::init()?;

    // create app and run it
    let app = AppState {
        pokemon_list: PokemonListState::new(Rc::new(bundle)).path(assets_dir),
        ..Default::default()
    };

    run_app(&mut tui.terminal, app)?;
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
    let pokemon: Vec<PokemonEntity> =
        from_str(include_str!("../data/data.json")).expect("load pokemon data error");
    let ability: AbilityMap =
        from_str(include_str!("../data/ability.json")).expect("load ability data error");
    Ok((pokemon, ability))
}
