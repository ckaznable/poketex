use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::widget::{
    dex::PokemonDexBlock,
    pmlist::{PokemonList, PokemonListStatus},
};

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

    // pm list
    f.render_stateful_widget(PokemonList::default(), right_chunks[0], pm_dex);
}
