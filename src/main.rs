mod ability;
mod args;
mod constant;
mod env;
mod pokemon;
mod ui;
mod util;
mod widget;

use ability::Ability;
use clap::Parser;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use env::DEF_LOCALES;
use pokemon::*;
use serde_json::from_str;
use std::{collections::HashMap, error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tui_input::{backend::crossterm::EventHandler, Input};
use widget::pmlist::PokemonListStatus;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct AppState {
    pm: PokemonListStatus,
    input_mode: InputMode,
    input: Input,
    query: String,
    no: String,
    ability: HashMap<String, Ability>,
    go_top: bool,
}

impl AppState {
    fn new(pm: PokemonListStatus, ability: HashMap<String, Ability>) -> Self {
        AppState {
            pm,
            ability,
            input_mode: InputMode::Normal,
            input: Input::default(),
            query: String::from(""),
            no: String::from(""),
            go_top: false,
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
}

fn get_pokemon_data() -> Result<Vec<Pokemon>, serde_json::Error> {
    let contents = include_str!("data/data.json");
    let pokemon: Result<Vec<Pokemon>, serde_json::Error> = from_str(&contents);
    pokemon
}

fn get_ability_data() -> Result<HashMap<String, Ability>, serde_json::Error> {
    let contents = include_str!("data/ability.json");
    let map: Result<HashMap<String, Ability>, serde_json::Error> = from_str(&contents);
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
            print!("ability data error");
            std::process::exit(2);
        }
    };

    let pokemon = match get_pokemon_data() {
        Ok(r) => r,
        Err(_) => {
            print!("pokemon data error");
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
    const PAGE_NUM: u8 = 4;

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            match event {
                // handle key with control
                KeyEvent {
                    code: c,
                    modifiers: KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => {
                    match c {
                        KeyCode::Char('f') => app.pm.scroll_down(PAGE_NUM),
                        KeyCode::Char('b') => app.pm.scroll_up(PAGE_NUM),
                        _ => {
                            continue;
                        }
                    }
                    app.cancel_last_cmd();
                }

                // handle number key
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: _,
                    kind: _,
                    state: _,
                } if ('0'..='9').contains(&c) => {
                    app.no(app.no.clone() + &c.to_string());
                    app.go_top(false);
                }

                // handle other key
                KeyEvent {
                    code: c,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    match c {
                        KeyCode::Char('q') => return Ok(()),

                        KeyCode::Down => app.pm.next(),
                        KeyCode::Char('j') => app.pm.next(),
                        KeyCode::PageDown => app.pm.scroll_down(PAGE_NUM),

                        KeyCode::Up => app.pm.previous(),
                        KeyCode::Char('k') => app.pm.previous(),
                        KeyCode::PageUp => app.pm.scroll_up(PAGE_NUM),

                        KeyCode::Left => app.pm.dex.previous(),
                        KeyCode::Char('h') => app.pm.dex.previous(),

                        KeyCode::Right => app.pm.dex.next(),
                        KeyCode::Char('l') => app.pm.dex.next(),

                        KeyCode::Char('/') => app.input_mode = InputMode::Editing,
                        KeyCode::Esc => app.query(String::from("")),

                        KeyCode::Home => app.jump(1),
                        KeyCode::End => app.jump(app.pm.items.len()),

                        KeyCode::Char('g') => {
                            if app.go_top {
                                app.jump(1);
                                app.go_top(false);
                            } else {
                                app.go_top(true);
                            }
                        }
                        KeyCode::Char('G') => {
                            if app.no.eq("") {
                                app.jump(app.pm.items.len());
                            } else {
                                let index = app.no.trim().parse::<usize>().unwrap();
                                app.jump(index);
                            }

                            app.no(String::from(""));
                            app.go_top(false);
                        }

                        _ => {}
                    }

                    app.cancel_last_cmd();
                }
            }

            match app.input_mode {
                InputMode::Editing => match event.code {
                    KeyCode::Enter => {
                        app.query(app.input.value().to_owned());
                        app.reset();
                    }
                    KeyCode::Esc => {
                        app.reset();
                        app.query(String::from(""));
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(event));
                        app.query(app.input.value().to_owned());
                    }
                },
                _ => {}
            }
        }
    }
}
