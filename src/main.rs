use anyhow::Result;
use std::{
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

#[cfg(unix)]
use xdg::BaseDirectories;

use clap::Parser;
use poketex::{
    env::DEF_LOCALES,
    keybinding::handle_key,
    pokemon::{
        ascii_form::{AsciiForms, AsciiJson},
        AbilityMap, PokemonBundle, PokemonEntity,
    },
    state::{AppState, PokemonListState},
    ui::ui,
};
use ratatui::crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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
    fn init() -> Result<Self> {
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
        if ratatui::crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = disable_raw_mode();
            let _ = self.terminal.show_cursor();
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let assets_dir = get_assets_dir_path()?;

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

    let Ok((pokemon, ability, ascii)) = load_data() else {
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
        pokemon_list: PokemonListState::new(Rc::new(bundle), AsciiForms::from(ascii))
            .path(assets_dir),
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

fn load_data() -> Result<(Vec<PokemonEntity>, AbilityMap, AsciiJson), ()> {
    let pokemon: Vec<PokemonEntity> =
        from_str(include_str!("../data/data.json")).expect("load pokemon data error");
    let ability: AbilityMap =
        from_str(include_str!("../data/ability.json")).expect("load ability data error");
    let ascii: AsciiJson =
        from_str(include_str!("../data/ascii.json")).expect("load ascii data error");
    Ok((pokemon, ability, ascii))
}

fn get_assets_dir_path() -> Result<PathBuf> {
    let assets_path = Path::new("colorscripts/small");

    // binary execute path
    if let Ok(execute_path) = std::env::current_exe() {
        if let Some(execute_dir) = execute_path.parent() {
            let assets_dir = execute_dir.join(assets_path);
            if assets_dir.exists() {
                return Ok(assets_dir);
            }
        };
    };

    // xdg data home
    #[cfg(unix)]
    if let Ok(xdg_dir) = BaseDirectories::new() {
        let data_home = xdg_dir.get_data_home().join("poketex");
        let assets_dir = data_home.join(assets_path);
        if assets_dir.exists() {
            return Ok(assets_dir);
        }
    };

    let usr_dir = Path::new("/usr/share/poketex");
    let assets_dir = usr_dir.join(assets_path);
    if assets_dir.exists() {
        return Ok(assets_dir);
    }

    let usr_dir = Path::new("/usr/local/share/poketex");
    let assets_dir = usr_dir.join(assets_path);
    if assets_dir.exists() {
        return Ok(assets_dir);
    }

    // default current dir
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.join(assets_path))
}
