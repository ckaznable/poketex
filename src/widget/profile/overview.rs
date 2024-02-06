use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Widget},
};

use crate::pokemon::{PokemonType, PokemonTypeKind};

pub struct Overview {
    pub name: String,
    pub pm_type: PokemonType,
}

impl Overview {
    pub fn new(name: String, pm_type: PokemonType) -> Self {
        Self { name, pm_type }
    }
}

impl Widget for Overview {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut type_span = vec![
            Span::from(self.name + " "),
            Span::styled(
                self.pm_type.0.to_string(),
                Style::default().bg(self.pm_type.0.color()).fg(Color::White),
            ),
            Span::from(" "),
        ];

        if let Some(kind) = self.pm_type.1 {
            if kind != PokemonTypeKind::Other {
                type_span.push(Span::styled(
                    kind.to_string(),
                    Style::default().bg(kind.color()).fg(Color::White),
                ));
            }
        }

        Block::default()
            .title(type_span)
            .borders(Borders::NONE)
            .render(area, buf);
    }
}
