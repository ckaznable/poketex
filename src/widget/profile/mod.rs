mod ability;
mod iv;
mod overview;

use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
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

        let area_height = area.height;
        let area_width = area.width;
        let region_form_page_num = state.region_form_len();
        let show_ability = area_height > 19;

        let lowercase_name = profile
            .default_name()
            .replace("Galarian form", "galar")
            .replace("Alola Form", "alola")
            .replace("Hisuian form", "hisui")
            .replace("Paldea form", "paldea")
            .replace("Meowscarada", "meowth-galar")
            .replace(" - ", "-")
            .replace(' ', "-")
            .to_lowercase();

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
                Constraint::Length(if show_ability { 1 } else { 0 }),
                Constraint::Min(0),
                Constraint::Length(if region_form_page_num > 1 { 1 } else { 0 }),
            ],
        )
        .split(area);

        let iv_status_constraint = if area_width < 35 {
            Constraint::Length(0)
        } else {
            Constraint::Min(0)
        };

        let show_layout2_ability = !show_ability && area_width > 100;
        let layout2_ability_length = if show_layout2_ability { 40 } else { 0 };
        let layout2_ability_margin = if show_layout2_ability { 1 } else { 0 };
        let layout2 = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(ansi_width),
                iv_status_constraint,
                Constraint::Length(layout2_ability_margin),
                Constraint::Length(layout2_ability_length),
            ],
        )
        .split(layout[2]);

        Overview::new(profile.name.get(), profile.r#type).render(layout[0], buf);

        let is_show_ability = show_ability || show_layout2_ability;

        let mut render_default_iv_ability = |buf: &mut Buffer| {
            IVStatus::new(profile.iv).render(layout2[1], buf);

            // ability at bottom
            if is_show_ability {
                let ability_area = if show_layout2_ability {
                    layout2[3]
                } else {
                    layout[4]
                };

                AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
                    ability_area,
                    buf,
                    &mut state.desc_scrollbar_state,
                );
            }
        };

        match ansi {
            Some(ansi) => {
                let ansi_height = ansi.height();
                Paragraph::new(ansi).render(layout2[0], buf);

                if ansi_height > 15 && area_height <= 25 {
                    let layout = Layout::new(
                        Direction::Vertical,
                        [Constraint::Length(10), Constraint::Min(0)],
                    )
                    .split(layout2[1]);

                    IVStatus::new(profile.iv).render(layout[0], buf);
                    // ability at right side bottom
                    AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
                        layout[1],
                        buf,
                        &mut state.desc_scrollbar_state,
                    );
                } else {
                    render_default_iv_ability(buf);
                }
            }
            None => {
                render_default_iv_ability(buf);
            }
        }

        if region_form_page_num > 1 {
            let title = format!(
                "<- {} / {} ->",
                state.profile_page + 1,
                region_form_page_num,
            );

            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[5], buf);
        }
    }
}
