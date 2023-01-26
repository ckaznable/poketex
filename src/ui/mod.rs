pub mod pokemon;

mod iv;
mod util;

use tui::{backend::Backend, Frame, layout::{Constraint, Layout, Direction, Alignment, Rect}, widgets::{Block, Borders, BorderType, ListItem, List}, text::{Span, Spans}, style::{Style, Color, Modifier}};

use crate::pokemon::Pokemon;

use self::{pokemon::PokemonList, util::{get_pokemon_iv_highest, get_type_bg_color}, iv::set_iv_layout};

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

fn set_type_block<B: Backend>(pm: &Pokemon, area: Rect, f: &mut Frame<B>) {
    let t = pm.r#type.get(0).unwrap();
    let mut type_span = vec![
        Span::styled(t, Style::default().bg(get_type_bg_color(t)).fg(Color::White)),
        Span::from(" "),
    ];

    match pm.r#type.get(1) {
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
    f.render_widget(block, area);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, pm_list: &mut PokemonList) {
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

    let mut left_constraints = vec![];
    for _ in 0..13 { left_constraints.push(Constraint::Length(1)) }
    left_constraints.push(Constraint::Min(0));

    // left inner blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(left_constraints.as_ref())
        .split(chunks[0]);

    // pm type block
    set_type_block(&current_pm, left_chunks[0], f);

    // HP
    set_iv_layout(
        "HP",
        &current_pm.iv.hp,
        max_iv,
        left_chunks[2],
        f
    );

    // Atk
    set_iv_layout(
        "Atk",
        &current_pm.iv.att,
        max_iv,
        left_chunks[4],
        f
    );

    // Def
    set_iv_layout(
        "Def",
        &current_pm.iv.def,
        max_iv,
        left_chunks[6],
        f
    );

    // S.Atk
    set_iv_layout(
        "S.Atk",
        &current_pm.iv.s_att,
        max_iv,
        left_chunks[8],
        f
    );

    // S.Def
    set_iv_layout(
        "S.Def",
        &current_pm.iv.s_def,
        max_iv,
        left_chunks[10],
        f
    );

    // Spd
    set_iv_layout(
        "Spd",
        &current_pm.iv.spd,
        max_iv,
        left_chunks[12],
        f
    );

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