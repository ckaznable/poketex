use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    state::{AppState, InputMode},
    widget::{
        filter::Filter, help::Help, pokemon_list::PokemonList, profile::PokemonProfileWidget,
    },
};

pub fn ui(f: &mut Frame, app: &mut AppState) {
    let size = f.size();

    // Surrounding block
    let block = Block::default();
    f.render_widget(block, size);

    // left chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(f.size());

    let block = PokemonProfileWidget;
    f.render_stateful_widget(block, chunks[0], &mut app.pokemon_list);

    // right chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(match app.tui.input_mode {
                InputMode::Normal => 1,
                InputMode::Editing => 3,
            }),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // search input
    f.render_stateful_widget(Filter, chunks[0], app);

    // pm list
    f.render_stateful_widget(PokemonList, chunks[1], &mut app.pokemon_list);

    // search input cursor
    if let Some((x, y)) = app.tui.cursor {
        f.set_cursor(x, y)
    };

    if app.tui.show_help {
        let area = centered_rect(50, 70, size);
        f.render_widget(Help, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let padding = (100 - percent_y) / 2;
    let layout = Layout::new(
        Direction::Vertical,
        Constraint::from_percentages([padding, percent_y, padding]),
    )
    .split(r);

    let padding = (100 - percent_x) / 2;
    Layout::new(
        Direction::Horizontal,
        Constraint::from_percentages([padding, percent_x, padding]),
    )
    .split(layout[1])[1]
}
