use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::{pokemon::PokemonIV, AppState};

use super::{ability::AbilityInfo, iv::IVStatus, topinfo::TopInfo};

#[derive(Clone)]
pub struct PokemonDex {
    pub name: String,
    pub pm_type: (String, Option<String>),
    pub iv: PokemonIV,
    pub ability: Vec<u16>,
}

pub struct PokemonDexState {
    pub items: Vec<PokemonDex>,
    pub page: usize,
}

impl Default for PokemonDexState {
    fn default() -> Self {
        PokemonDexState {
            items: vec![],
            page: 1,
        }
    }
}

impl PokemonDexState {
    pub fn current(&self) -> &PokemonDex {
        &self.items[self.page - 1]
    }

    pub fn next(&mut self) {
        if self.page < self.items.len() {
            self.page += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.page > 1 {
            self.page -= 1;
        }
    }
}

pub struct PokemonDexBlock;

impl Default for PokemonDexBlock {
    fn default() -> Self {
        PokemonDexBlock
    }
}

impl StatefulWidget for PokemonDexBlock {
    type State = AppState;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let dex = &state.pm.dex;
        let current = dex.current();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(11),
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        // pm type block
        TopInfo::new(current.name.clone(), current.pm_type.clone()).render(layout[0], buf);

        IVStatus::new(current.iv).render(layout[2], buf);

        AbilityInfo::new(current.ability.clone(), state.ability.clone()).render(layout[4], buf);

        let title = format!(
            "<- {} / {} ->",
            dex.page.to_string().as_str(),
            dex.items.len().to_string().as_str()
        );

        if dex.items.len() > 1 {
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[5], buf);
        }
    }
}
