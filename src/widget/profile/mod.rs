mod ability;
mod iv;
mod overview;

use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::state::{pokemon::AsciiType, PokemonListState};

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

        let ascii_form = state
            .ascii_form_map
            .get(&lowercase_name)
            .and_then(|forms| forms.get(state.ascii_form_index % forms.len()))
            .map_or("", |f| f);

        let ascii_type = if ascii_form == "shiny" {
            AsciiType::Shiny
        } else {
            AsciiType::Normal
        };

        let ascii_form = if ascii_form == "regular" || ascii_form == "shiny" {
            String::from("")
        } else if !ascii_form.is_empty() {
            format!("-{}", ascii_form)
        } else {
            ascii_form.to_string()
        };

        let (ansi_width, ansi_height, ansi) = std::fs::read(
            state
                .get_assets_path(ascii_type)
                .join(lowercase_name + &ascii_form),
        )
        .map(|buffer| buffer.into_text().ok())
        .ok()
        .flatten()
        .map_or((0u16, 0u16, None), |ansi| {
            (
                ansi.width() as u16 + 1,
                ansi.height() as u16 + 1,
                Some(ansi),
            )
        });

        let [overview, _, main, _, ability_bottom_area, region_form_navigation] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(std::cmp::max(ansi_height, 12)),
                Constraint::Length(if show_ability { 1 } else { 0 }),
                Constraint::Min(0),
                Constraint::Length(if region_form_page_num > 1 { 1 } else { 0 }),
            ])
            .areas(area);

        let iv_status_constraint = if area_width < 35 {
            Constraint::Length(0)
        } else {
            Constraint::Min(0)
        };

        let show_main_ability = !show_ability && area_width > 100;
        let main_right_side_length = if show_main_ability { 40 } else { 0 };
        let main_right_side_margin = if show_main_ability { 1 } else { 0 };
        let [ansi_area, main_iv, _, main_ability] = Layout::horizontal([
            Constraint::Length(ansi_width),
            iv_status_constraint,
            Constraint::Length(main_right_side_margin),
            Constraint::Length(main_right_side_length),
        ])
        .areas(main);

        Overview::new(profile.name.get(), profile.r#type).render(overview, buf);

        let is_show_ability = show_ability || show_main_ability;

        let mut render_default_iv_ability = |buf: &mut Buffer| {
            IVStatus::new(profile.iv).render(main_iv, buf);

            // ability at bottom
            if is_show_ability {
                let ability_bottom_area = if show_main_ability {
                    main_ability
                } else {
                    ability_bottom_area
                };

                AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
                    ability_bottom_area,
                    buf,
                    &mut state.desc_scrollbar_state,
                );
            }
        };

        match ansi {
            Some(ansi) => {
                Paragraph::new(ansi).render(ansi_area, buf);

                if ansi_height > 15 && area_height <= 25 {
                    let [iv_area, ability_bottom_area] =
                        Layout::vertical([Constraint::Length(11), Constraint::Min(0)])
                            .areas(main_iv);

                    IVStatus::new(profile.iv).render(iv_area, buf);
                    // ability at right side bottom
                    AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
                        ability_bottom_area,
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
                .render(region_form_navigation, buf);
        }
    }
}
