mod iv;
mod overview;
mod ability;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Widget},
};

use crate::state::PokemonListState;

use self::ability::AbilityParaGraph;

use {iv::IVStatus, overview::Overview};

pub struct PokemonProfileWidget<'a>(pub &'a PokemonListState<'a>);

impl<'a> Widget for PokemonProfileWidget<'a> {
    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
    ) {
        let Some(profile) = self.0.profile_with_region_form() else {
            return;
        };

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

        Overview {
            name: profile.name.get().to_string(),
            pm_type: profile.r#type.clone()
        }.render(layout[0], buf);

        IVStatus::new(profile.iv).render(layout[2], buf);

        AbilityParaGraph(self.0.bundle.get_ability_text(&profile)).render(layout[4], buf);

        let page_num = self.0.region_form_len();
        if page_num > 1 {
            let title = format!(
                "<- {} / {} ->",
                self.0.profile_page,
                page_num,
            );

            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[5], buf);
        }
    }
}
