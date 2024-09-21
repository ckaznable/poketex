use ratatui::{
    layout::{Constraint, Layout, Rect},
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
    let constraint = if !app.tui.show_list {
        [Constraint::Percentage(100), Constraint::Length(0)]
    } else if area.width >= 55 {
        [Constraint::Min(0), Constraint::Length(25)]
    } else if area.width >= 100 {
        [Constraint::Min(0), Constraint::Length(40)]
    } else {
        [Constraint::Percentage(100), Constraint::Length(0)]
    };

    let [left, right] = Layout::horizontal(constraint).margin(2).areas(f.size());

    // left chunks
    f.render_stateful_widget(PokemonProfileWidget(app.tui), left, &mut app.pokemon_list);

    // right chunks
    if right.width >= 25 {
        let [search, pm_list] = Layout::vertical([
            Constraint::Length(match app.tui.input_mode {
                InputMode::Normal => 1,
                InputMode::Editing => 3,
            }),
            Constraint::Min(0),
        ])
        .areas(right);

        // search input
        f.render_stateful_widget(Filter, search, app);

        // pm list
        f.render_stateful_widget(PokemonList, pm_list, &mut app.pokemon_list);

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
    let [_, v_center] =
        Layout::vertical(Constraint::from_percentages([padding, percent_y])).areas(r);

    let padding = (100 - percent_x) / 2;
    let [_, center] =
        Layout::horizontal(Constraint::from_percentages([padding, percent_x])).areas(v_center);
    center
}
