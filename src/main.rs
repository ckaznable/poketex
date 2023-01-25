use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Modifier, Color},
    widgets::{Block, BorderType, Borders, ListState, List, ListItem, Gauge},
    Frame, Terminal, text::{Spans, Span},
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

struct PokemonList {
    state: ListState,
    items: Vec<Pokemon>,
}

impl PokemonList {
    fn new(mut items: Vec<Pokemon>) -> PokemonList {
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
        let mut state = ListState::default();
        state.select(Some(0));

        PokemonList {
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

fn get_pokemon_iv_highest(pm: &PokemonIV) -> u16 {
    *vec![pm.hp, pm.att, pm.def, pm.s_att, pm.s_def, pm.spd].iter().max().unwrap()
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
    let pm_list = PokemonList::new(pokemon);
    let res = run_app(&mut terminal, pm_list);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut pm_list: PokemonList) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut pm_list))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => pm_list.next(),
                KeyCode::Up => pm_list.previous(),
                _ => {}
            }
        }
    }
}

fn data_list<'a>(pm_list: &&'a mut PokemonList) -> List<'a> {
    let items: Vec<ListItem> = pm_list.items
        .iter()
        .map(|item| {
            let title = "#".to_string()
                + item.no.to_string().as_str()
                + " "
                + item.name.en.as_str();

            ListItem::new(vec![Spans::from(title)])
        })
        .collect();

    List::new(items)
        .block(Block::default()
        .borders(Borders::LEFT)
        .title_alignment(Alignment::Center)
        .title("Pokemon List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
}

fn get_type_bg_color(t: &str) -> Color {
    match t {
        "fire" => Color::Rgb(255, 68, 34),
        "grass" => Color::Rgb(119, 204, 85),
        "water" => Color::Rgb(51, 153, 255),
        "normal" => Color::Rgb(187, 187, 170),
        "electric" => Color::Rgb(255, 204, 51),
        "ice" => Color::Rgb(119, 221, 255),
        "fighting" => Color::Rgb(187, 85, 68),
        "poison" => Color::Rgb(170, 85, 153),
        "ground" => Color::Rgb(221, 187, 85),
        "flying" => Color::Rgb(102, 153, 255),
        "psychic" => Color::Rgb(255, 85, 153),
        "bug" => Color::Rgb(170, 187, 34),
        "rock" => Color::Rgb(187, 170, 102),
        "ghost" => Color::Rgb(102, 102, 187),
        "dragon" => Color::Rgb(119, 102, 238),
        "dark" => Color::Rgb(119, 85, 68),
        "steel" => Color::Rgb(170, 170, 187),
        "fairy" => Color::Rgb(255, 170, 255),
        _ => Color::Black
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, pm_list: &mut PokemonList) {
    let size = f.size();
    let current_pm = pm_list.get_current_item();
    let max_iv = get_pokemon_iv_highest(&current_pm.iv) as f32;

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
        .constraints([Constraint::Percentage(66), Constraint::Percentage(34)].as_ref())
        .split(f.size());

    // left inner blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0)
        ].as_ref())
        .split(chunks[0]);

    let hp_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[2]);

    let block = Block::default().title("HP");
    f.render_widget(block, hp_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.hp as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.hp.to_string());
    f.render_widget(gauge, hp_chunks[1]);

    let atk_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[4]);

    let block = Block::default().title("Atk");
    f.render_widget(block, atk_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.att as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.att.to_string());
    f.render_widget(gauge, atk_chunks[1]);

    let def_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[6]);

    let block = Block::default().title("Def");
    f.render_widget(block, def_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.def as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.def.to_string());
    f.render_widget(gauge, def_chunks[1]);

    let s_atk_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[8]);

    let block = Block::default().title("S.Atk");
    f.render_widget(block, s_atk_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.s_att as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.s_att.to_string());
    f.render_widget(gauge, s_atk_chunks[1]);

    let s_def_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[10]);

    let block = Block::default().title("S.Def");
    f.render_widget(block, s_def_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.s_def as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.s_def.to_string());
    f.render_widget(gauge, s_def_chunks[1]);

    let spd_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(left_chunks[12]);

    let block = Block::default().title("Spd");
    f.render_widget(block, spd_chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((current_pm.iv.spd as f32 / max_iv) * 100.0) as u16)
        .label(current_pm.iv.spd.to_string());
    f.render_widget(gauge, spd_chunks[1]);

    let t = current_pm.r#type.get(0).unwrap();
    let mut type_span = vec![
        Span::styled(t, Style::default().bg(get_type_bg_color(t)).fg(Color::White)),
        Span::from(" "),
    ];

    match current_pm.r#type.get(1) {
        Some(t) => {
            if !t.eq("unknow") {
                type_span.push(Span::styled(t, Style::default().bg(get_type_bg_color(t)).fg(Color::White)));
            }
            ()
        },
        None => (),
    };

    let block = Block::default()
        .title(type_span)
        .borders(Borders::NONE);
    f.render_widget(block, left_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let right_chunks_margin = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .split(right_chunks[0]);

    // pm list
    f.render_stateful_widget(data_list(&pm_list), right_chunks_margin[0], &mut pm_list.state);
}