use anyhow::anyhow;
use std::{
    error::Error,
    io::{self, Cursor},
    rc::Rc,
};

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poketex::{
    assets::load_low_quality_img,
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
use ratatui_image::{picker::Picker, protocol::ResizeProtocol};
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

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

    let Ok((pokemon, ability)) = load_data() else {
        println!("load data error");
        std::process::exit(2);
    };

    let Ok(assets) = load_assets() else {
        panic!("load assets error");
    };

    let bundle = PokemonBundle {
        ability: Rc::new(ability),
        pokemon: pokemon.into_iter().map(Rc::new).collect(),
    };

    // setup terminal
    let mut tui = Tui::init()?;

    // create app and run it
    let pokemon_list = PokemonListState::new(Rc::new(bundle)).with_assets(assets);
    let app = AppState {
        pokemon_list,
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
        from_str(include_str!("../assets/data/data.json")).expect("load pokemon data error");
    let ability: AbilityMap =
        from_str(include_str!("../assets/data/ability.json")).expect("load ability data error");
    Ok((pokemon, ability))
}

fn load_assets() -> anyhow::Result<Vec<Box<dyn ResizeProtocol>>> {
    let raws = load_low_quality_img();
    decode_images(raws)
}

fn decode_images(raws: Vec<Vec<u8>>) -> anyhow::Result<Vec<Box<dyn ResizeProtocol>>> {
    let mut picker = Picker::new((8, 12));
    picker.guess_protocol();

    let len = raws.len();
    let images = raws
        .into_iter()
        .filter_map(|raw| {
            let cursor = Cursor::new(raw);
            let dyn_img = image::io::Reader::new(cursor)
                .with_guessed_format()
                .ok()?
                .decode()
                .ok()?;

            Some(picker.new_resize_protocol(dyn_img))
        })
        .collect::<Vec<_>>();

    if len != images.len() {
        Err(anyhow!("load assets error"))
    } else {
        Ok(images)
    }
}
