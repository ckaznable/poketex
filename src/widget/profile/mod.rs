mod ability;
mod iv;
mod overview;

use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::state::PokemonListState;

use self::ability::AbilityParaGraph;

use {iv::IVStatus, overview::Overview};

pub struct PokemonProfileWidget;

impl StatefulWidget for PokemonProfileWidget {
    type State = PokemonListState;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let Some(profile) = state.profile_with_region_form() else {
            return;
        };

        let lowercase_name = profile.name.en.to_lowercase();
        let (ansi_width, ansi_height, ansi) =
            match std::fs::read(state.asset_path.join(lowercase_name)) {
                Err(_) => (0u16, 0u16, None),
                Ok(buffer) => match buffer.into_text() {
                    Ok(ansi) => (
                        ansi.width() as u16 + 1,
                        ansi.height() as u16 + 1,
                        Some(ansi),
                    ),
                    Err(_) => (0u16, 0u16, None),
                },
            };

        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(std::cmp::max(ansi_height, 11)),
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ],
        )
        .split(area);

        Overview::new(profile.name.get(), profile.r#type).render(layout[0], buf);

        let ansi_and_vi = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(ansi_width), Constraint::Min(0)],
        )
        .split(layout[2]);

        IVStatus::new(profile.iv).render(ansi_and_vi[1], buf);
        if let Some(ansi) = ansi {
            Paragraph::new(ansi).render(ansi_and_vi[0], buf);
        }

        AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
            layout[4],
            buf,
            &mut state.desc_scrollbar_state,
        );

        let page_num = state.region_form_len();
        if page_num > 1 {
            let title = format!("<- {} / {} ->", state.profile_page + 1, page_num,);

            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[5], buf);
        }
    }
}
