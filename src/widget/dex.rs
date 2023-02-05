use tui::{
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{StatefulWidget, Widget, Block},
};

use crate::pokemon::PokemonIV;

use super::{iv::IVStatus, topinfo::TopInfo};

#[derive(Clone)]
pub struct PokemonDex {
    pub name: String,
    pub pm_type: (String, Option<String>),
    pub iv: PokemonIV,
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
    type State = PokemonDexState;

    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(11),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        let current = state.current();
        // pm type block
        TopInfo::new(current.name.clone(), current.pm_type.clone())
            .render(layout[0], buf);

        IVStatus::new(current.iv).render(layout[2], buf);

        let title = format!("<- {} / {} ->", state.page.to_string().as_str(), state.items.len().to_string().as_str());
        if state.items.len() > 1 {
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[4], buf);
        }
    }
}
