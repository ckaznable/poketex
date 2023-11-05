use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Widget},
};

use crate::pokemon::pokemon::PokemonIV;

pub struct IVStatusBar<'a> {
    title: &'a str,
    value: u16,
    max: f32,
}

impl<'a> IVStatusBar<'a> {
    pub fn new(title: &'a str, value: u16, max: f32) -> Self {
        IVStatusBar { title, value, max }
    }
}

impl<'a> Widget for IVStatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut constraints = vec![];
        for _ in 0..12 {
            constraints.push(Constraint::Length(1))
        }

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(60),
                Constraint::Min(0),
            ])
            .split(area);

        Block::default().title(self.title).render(layout[0], buf);

        Gauge::default()
            .block(Block::default().borders(Borders::NONE))
            .gauge_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(((self.value as f32 / self.max) * 100.0) as u16)
            .label(self.value.to_string())
            .render(layout[1], buf);
    }
}

#[derive(Copy, Clone, Default)]
pub struct IVStatus {
    iv: PokemonIV,
}

impl IVStatus {
    pub fn new(iv: PokemonIV) -> IVStatus {
        IVStatus { iv }
    }

    pub fn get_pokemon_iv_highest(&self) -> f32 {
        *[
            self.iv.hp,
            self.iv.att,
            self.iv.def,
            self.iv.s_att,
            self.iv.s_def,
            self.iv.spd,
        ]
        .iter()
        .max()
        .unwrap() as f32
    }
}

impl Widget for IVStatus {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut constraints = vec![];
        for _ in 0..11 {
            constraints.push(Constraint::Length(1))
        }

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints::<Vec<_>>(constraints)
            .split(area);

        let max = self.get_pokemon_iv_highest();

        IVStatusBar::new("HP", self.iv.hp, max).render(layout[0], buf);
        IVStatusBar::new("Atk", self.iv.att, max).render(layout[2], buf);
        IVStatusBar::new("Def", self.iv.def, max).render(layout[4], buf);
        IVStatusBar::new("S.Atk", self.iv.s_att, max).render(layout[6], buf);
        IVStatusBar::new("S.Def", self.iv.s_def, max).render(layout[8], buf);
        IVStatusBar::new("Spd", self.iv.spd, max).render(layout[10], buf);
    }
}
