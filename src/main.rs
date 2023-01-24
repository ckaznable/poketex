use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Modifier},
    widgets::{Block, BorderType, Borders, TableState, Row, Table},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Clone)]
struct PokemonName {
    zh: String,
    en: String,
    jp: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct PokemonIV {
    hp: u16,
    att: u16,
    def: u16,
    s_att: u16,
    s_def: u16,
    spd: u16
}

#[derive(Serialize, Deserialize, Clone)]
struct Pokemon {
    no: u16,
    r#type: Vec<String>,
    name: PokemonName,
    iv: PokemonIV,
}

struct PokemonTable {
    state: TableState,
    items: Vec<Pokemon>,
}

impl PokemonTable {
    fn new(mut items: Vec<Pokemon>) -> PokemonTable {
        // make sure items has def pokemon
        if items.len() == 0 {
            let pm = Pokemon {
                name: PokemonName { zh: "".to_string(), en: "".to_string(), jp: "".to_string() },
                no: 0,
                r#type: vec!["unknow".to_string()],
                iv: PokemonIV {
                    hp: 0,
                    att: 0,
                    def: 0,
                    s_att: 0,
                    s_def: 0,
                    spd: 0
                }
            };

            items.push(pm);
        };

        // init position = 0
        let mut state = TableState::default();
        state.select(Some(0));

        PokemonTable {
            state,
            items,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn get_index(&self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn get_current_item(&self) -> &Pokemon {
        let i = self.get_index();
        match self.items.get(i) {
            Some(pm) => pm,
            None => self.items.get(0).unwrap(),
        }
    }
}

fn get_pokemon_data() -> Result<Vec<Pokemon>, serde_json::Error> {
    let contents = include_str!("data.json");
    let pokemon: Result<Vec<Pokemon>, serde_json::Error> = from_str(&contents);
    pokemon
}

fn main() -> Result<(), Box<dyn Error>> {
    let pokemon = match get_pokemon_data() {
        Ok(r) => r,
        Err(_) => {
            print!("data source error");
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
    let pm_table = PokemonTable::new(pokemon);
    let res = run_app(&mut terminal, pm_table);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut pm_table: PokemonTable) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut pm_table))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => pm_table.next(),
                KeyCode::Up => pm_table.previous(),
                _ => {}
            }
        }
    }
}

fn get_table<'a>(pm_table: &&'a mut PokemonTable) -> Table<'a> {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = pm_table.items.iter().map(|item| {
        Row::new(vec![
            "#".to_string()
            + item.no.to_string().as_str()
            + " "
            + item.name.en.as_str()
        ]).height(1).bottom_margin(1)
    });

    Table::new(rows)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(100)])
}

fn ui<B: Backend>(f: &mut Frame<B>, pm_table: &mut PokemonTable) {
    let size = f.size();
    let current_pm = pm_table.get_current_item();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("poketex")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // left inner blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    let block = Block::default()
        .title(current_pm.name.en.as_str())
        .borders(Borders::RIGHT);
    f.render_widget(block, left_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let right_chunks_margin = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .split(right_chunks[0]);

    let t = get_table(&pm_table);
    f.render_stateful_widget(t, right_chunks_margin[0], &mut pm_table.state);
}