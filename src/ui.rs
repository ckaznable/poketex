use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::widget::{dex::PokemonDexBlock, pmlist::PokemonListStatus};

fn data_list<'a>(pm_dex: &&'a mut PokemonListStatus) -> List<'a> {
    let items: Vec<ListItem> = pm_dex
        .items
        .iter()
        .map(|item| {
            let title = "#".to_string()
                + item.no.to_string().as_str()
                + " "
                + item.name.get_name().as_str();

            ListItem::new(vec![Spans::from(title)])
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .title_alignment(Alignment::Center)
                .title("Pokemon List"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
}

pub fn ui<B: Backend>(f: &mut Frame<B>, pm_dex: &mut PokemonListStatus) {
    let size = f.size();

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

    let block = PokemonDexBlock::default();
    f.render_stateful_widget(block, chunks[0], &mut pm_dex.dex);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let right_chunks_margin = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .split(right_chunks[0]);

    // pm list
    f.render_stateful_widget(
        data_list(&pm_dex),
        right_chunks_margin[0],
        &mut pm_dex.state,
    );
}
