use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarOrientation, StatefulWidget},
};

use crate::{constant::LIST_H_MARGIN, state::PokemonListState};

pub struct PokemonList;

impl StatefulWidget for PokemonList {
    type State = PokemonListState;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(LIST_H_MARGIN)
            .split(area);

        let items: Vec<ListItem> = state
            .list_items()
            .iter()
            .map(|item| ListItem::new(vec![Line::from(item.name_with_no())]))
            .collect();

        List::new(items)
            .block(Block::default().borders(Borders::LEFT))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .render(layout[0], buf, &mut state.list_state);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().bg(Color::DarkGray))
            .render(layout[0], buf, &mut state.list_scrollbar_state);
    }
}
