use std::collections::HashMap;

use tui::{
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::ability::Ability;

pub struct AbilityInfo {
    pub ability: Vec<u16>,
    pub map: HashMap<String, Ability>,
}

impl AbilityInfo {
    pub fn new(ability: Vec<u16>, map: HashMap<String, Ability>) -> Self {
        AbilityInfo { ability, map }
    }

    pub fn get_ability_from_map(&self, id: u16) -> Option<&Ability> {
        self.map.get(&id.to_string())
    }
}

impl Widget for AbilityInfo {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let mut span: Vec<Spans> = vec![];
        let mut setter = |x: usize| match self.ability.get(x) {
            None => (),
            Some(id) => match self.get_ability_from_map(*id) {
                None => (),
                Some(a) => {
                    if x > 0 {
                        span.push(Spans::from(""));
                    }

                    span.push(Spans::from(a.name()));
                    span.push(Spans::from(a.desc()));
                }
            },
        };

        let len = self.ability.len();
        let j = if len.eq(&1) { 1 } else { len - 1 };
        for i in 0..j {
            setter(i);
        }

        Paragraph::new(span)
            .block(Block::default().title("Ability").borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .render(layout[0], buf);
    }
}
