use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget,
    },
};

use crate::{constant::LIST_H_MARGIN, pokemon::pokemon::PokemonEntity, state::AppState};

pub struct PokemonList<'a>(pub &'a AppState<'a>);

impl<'a> Widget for PokemonList<'a> {
    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(LIST_H_MARGIN)
            .split(area);

        let items: Vec<ListItem> = self.0.pokemon_list
            .list_item
            .iter()
            .map(|item| ListItem::new(vec![Line::from(item.name_with_no())]))
            .collect();

        ratatui::widgets::Widget::render(List::new(items)
            .block(Block::default().borders(Borders::LEFT))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ), layout[0], buf);
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().bg(Color::DarkGray))
            .render(layout[0], buf, &mut self.0.pokemon_list.list_scrollbar_state);
    }
}
