use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    state::{AppState, InputMode},
    widget::{
        filter::Filter, help::Help, pokemon_list::PokemonList, profile::PokemonProfileWidget,
    },
};

pub fn ui(f: &mut Frame, app: &mut AppState) {
    let area = f.size();
    let constraint = if area.width >= 55 {
        [Constraint::Min(0), Constraint::Length(25)]
    } else if area.width >= 100 {
        [Constraint::Min(0), Constraint::Length(40)]
    } else {
        [Constraint::Percentage(100), Constraint::Length(0)]
    };

    // left chunks
    let chunks = Layout::horizontal(constraint).margin(2).split(f.size());

    let block = PokemonProfileWidget;
    f.render_stateful_widget(block, chunks[0], &mut app.pokemon_list);

    // right chunks
    if chunks[1].width >= 25 {
        let chunks = Layout::vertical([
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
    }

    if app.tui.show_help {
        let area = centered_rect(50, 70, area);
        f.render_widget(Help, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let padding = (100 - percent_y) / 2;
    let layout =
        Layout::vertical(Constraint::from_percentages([padding, percent_y, padding])).split(r);

    let padding = (100 - percent_x) / 2;
    Layout::horizontal(Constraint::from_percentages([padding, percent_x, padding])).split(layout[1])
        [1]
}
