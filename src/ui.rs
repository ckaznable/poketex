use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    widget::{dex::PokemonDexBlock, filter::Filter, pmlist::PokemonList},
    AppState,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("poketex")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    // left chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(f.size());

    let block = PokemonDexBlock::default();
    f.render_stateful_widget(block, chunks[0], app);

    // right chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(match app.input_mode {
                crate::InputMode::Normal => 1,
                crate::InputMode::Editing => 3,
            }),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // search input
    f.render_stateful_widget(Filter::default(), chunks[0], app);

    // pm list
    f.render_stateful_widget(PokemonList::default(), chunks[1], app);
}
