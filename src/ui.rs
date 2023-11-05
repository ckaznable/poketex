use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    widget::{filter::Filter, help::Help, pokemon_list::PokemonList, profile::PokemonProfileWidget},
    state::{InputMode, AppState},
};

pub fn ui(f: &mut Frame, app: &mut AppState) {
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Poketex")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    // left chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(f.size());

    let block = PokemonProfileWidget(&app.pokemon_list);
    f.render_widget(block, chunks[0]);

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
    f.render_widget(Filter(&app), chunks[0]);

    // pm list
    f.render_widget(PokemonList(&app), chunks[1]);

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
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
