use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::pokemon::PokemonAbilityText;

pub struct AbilityParaGraph(pub Vec<PokemonAbilityText>);

impl Widget for AbilityParaGraph {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let span = self.0.iter().enumerate().fold(vec![], |mut line, (i, a)| {
            if i > 0 {
                line.push(Line::from(""));
            }

            line.push(Line::from(a.name.clone()));
            line.push(Line::from(a.desc.clone()));
            line
        });

        Paragraph::new(span)
            .block(Block::default().title("Ability").borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .render(layout[0], buf);
    }
}
