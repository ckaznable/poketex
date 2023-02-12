use std::collections::HashMap;

use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Widget},
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
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Min(0),
            ])
            .split(area);

        let mut setter = |x: usize| match self.ability.get(x) {
            None => (),
            Some(id) => match self.get_ability_from_map(*id) {
                None => (),
                Some(a) => {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(1), Constraint::Min(0)])
                        .split(layout[x]);

                    Block::default().title(a.name()).render(chunks[0], buf);
                    Block::default().title(a.desc()).render(chunks[1], buf);
                }
            },
        };

        setter(0);
        setter(1);
        setter(2);
    }
}
