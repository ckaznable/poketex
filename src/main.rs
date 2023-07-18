mod ability;
mod args;
mod constant;
mod env;
mod keybinding;
mod pokemon;
mod ui;
mod util;
mod widget;

use ability::Ability;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use env::DEF_LOCALES;
use keybinding::handle_key;
use pokemon::*;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal, widgets::ScrollbarState,
};
use serde_json::from_str;
use std::{collections::HashMap, error::Error, io};
use tui_input::Input;
use widget::pmlist::PokemonListStatus;

#[derive(Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct AppState {
    pm: PokemonListStatus,
    input_mode: InputMode,
    input: Input,
    query: String,
    no: String,
    ability: HashMap<String, Ability>,
    go_top: bool,
    show_help: bool,
    cursor: Option<(u16, u16)>,
    list_scrollbar_state: ScrollbarState,
}

impl AppState {
    fn new(pm: PokemonListStatus, ability: HashMap<String, Ability>) -> Self {
        let list_scrollbar_state = ScrollbarState::default()
            .content_length(pm.items.len() as u16);

        AppState {
            pm,
            ability,
            list_scrollbar_state,
            ..Default::default()
        }
    }

    fn reset(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input.reset();
    }

    fn query(&mut self, q: String) {
        self.query = q.clone();
        self.pm.set_list_filter(q);
    }

    fn no(&mut self, no: String) {
        self.no = no;
    }

    fn jump(&mut self, i: usize) {
        if i > 0 || i - 1 > self.pm.items.len() {
            self.pm.current(i - 1);
        }
    }

    fn go_top(&mut self, f: bool) {
        self.go_top = f;
    }

    fn cancel_last_cmd(&mut self) {
        self.no = String::from("");
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}

fn get_pokemon_data() -> Result<Vec<Pokemon>, serde_json::Error> {
    let contents = include_str!("data/data.json");
    let pokemon: Result<Vec<Pokemon>, serde_json::Error> = from_str(contents);
    pokemon
}

fn get_ability_data() -> Result<HashMap<String, Ability>, serde_json::Error> {
    let contents = include_str!("data/ability.json");
    let map: Result<HashMap<String, Ability>, serde_json::Error> = from_str(contents);
    map
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();

    unsafe {
        DEF_LOCALES = Box::leak(args.locale.into_boxed_str());
    }

    let ability = match get_ability_data() {
        Ok(r) => r,
        Err(_) => {
            println!("ability data error");
            std::process::exit(2);
        }
    };

    let pokemon = match get_pokemon_data() {
        Ok(r) => r,
        Err(_) => {
            println!("pokemon data error");
            std::process::exit(2);
        }
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
            if handle_key(&mut app, event).is_some() {
                return Ok(());
            }
        }
    }
}
