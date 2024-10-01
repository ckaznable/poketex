mod ability;
mod iv;
mod overview;

use std::rc::Rc;

use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::{
    pokemon::PokemonEntity,
    state::{pokemon::AsciiType, tui::TuiState, PokemonListState},
};

use self::ability::AbilityParaGraph;

use {iv::IVStatus, overview::Overview};

// [name with type, ansi, iv, ability, navigation]
type ProfileLayout = [Rect; 5];
// [ansi, iv, ability]
type ProfileBodyLayout = [Rect; 3];

#[derive(Copy, Clone)]
struct LayoutParam {
    ansi_height: u16,
    ansi_width: u16,
    show_page_navigation: bool,
}

pub struct PokemonProfileWidget(pub TuiState);

impl PokemonProfileWidget {
    const SPACE_WITHOUT_ANSI_H: u16 = 10;
    const SPACE_WITHOUT_ANSI_V: u16 = 3;

    fn get_render_areas(&self, area: Rect, param: LayoutParam) -> ProfileLayout {
        let [name, _, body, navi] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(if param.show_page_navigation { 1 } else { 0 }),
        ])
        .areas(area);

        let [ansi, iv, ability] = if (!self.0.show_abilities && !self.0.show_iv)
            || body.width <= param.ansi_width
                && body.height <= param.ansi_height
                && body.width.saturating_sub(param.ansi_width) < Self::SPACE_WITHOUT_ANSI_H
                && body.height.saturating_sub(param.ansi_height) < Self::SPACE_WITHOUT_ANSI_V
        {
            self.get_only_ansi_areas(body)
        } else if body.height.saturating_sub(param.ansi_height) < Self::SPACE_WITHOUT_ANSI_V
            && body.height.saturating_sub(12) < Self::SPACE_WITHOUT_ANSI_V
        {
            self.get_h_rect_areas(body, param)
        } else if body.width.saturating_sub(param.ansi_width) < Self::SPACE_WITHOUT_ANSI_H {
            self.get_v_rect_areas(body, param)
        } else {
            self.get_rect_areas(body, param)
        };

        [name, ansi, iv, ability, navi]
    }

    fn get_constraints_with_iv_ability(&self, remaining_space: u16) -> (Constraint, Constraint) {
        use Constraint::*;
        match (self.0.show_abilities, self.0.show_iv) {
            (true, false) => (Percentage(100), Length(0)),
            (false, true) => (Length(0), Percentage(100)),
            (false, false) => (Length(0), Length(0)),
            (true, true) => {
                let space: u16 = Self::SPACE_WITHOUT_ANSI_H;
                if remaining_space <= space {
                    (Length(space), Length(0))
                } else if remaining_space >= space * 6 {
                    (Percentage(40), Percentage(60))
                } else if remaining_space >= space * 2 {
                    (Percentage(50), Percentage(50))
                } else if remaining_space >= space {
                    (Min(0), Length(space / 2))
                } else {
                    (Length(space), Length(0))
                }
            }
        }
    }

    fn get_h_rect_areas(&self, body: Rect, param: LayoutParam) -> ProfileBodyLayout {
        let remaining_space = body.width.saturating_sub(param.ansi_width);
        let (iv, ability) = self.get_constraints_with_iv_ability(remaining_space);
        Layout::horizontal([Constraint::Length(param.ansi_width), iv, ability]).areas(body)
    }

    fn get_v_rect_areas(&self, body: Rect, param: LayoutParam) -> ProfileBodyLayout {
        let remaining_space = body.height.saturating_sub(param.ansi_height);
        let (iv, ability) = self.get_constraints_with_iv_ability(remaining_space);
        Layout::vertical([Constraint::Length(param.ansi_height), iv, ability]).areas(body)
    }

    fn get_rect_areas(&self, body: Rect, param: LayoutParam) -> ProfileBodyLayout {
        use Constraint::*;
        if body.height.saturating_sub(param.ansi_height) < 5 && body.height.saturating_sub(12) >= 5
        {
            let iv = if self.0.show_iv {
                Length(12)
            } else {
                Length(0)
            };
            let ability = if self.0.show_abilities {
                Min(0)
            } else {
                Length(0)
            };
            let [left, right] = Layout::horizontal([Length(param.ansi_width), Min(0)]).areas(body);
            let [top, bottom] = Layout::vertical([iv, ability]).areas(right);
            [left, top, bottom]
        } else {
            let iv = if self.0.show_iv { Min(0) } else { Length(0) };
            let ability = if self.0.show_abilities {
                Min(0)
            } else {
                Length(0)
            };
            let [top, bottom] =
                Layout::vertical([Length(param.ansi_height.max(12)), ability]).areas(body);
            let [left, right] = Layout::horizontal([Length(param.ansi_width), iv]).areas(top);
            [left, right, bottom]
        }
    }

    fn get_only_ansi_areas(&self, body: Rect) -> ProfileBodyLayout {
        Layout::vertical([
            Constraint::Percentage(100),
            Constraint::Length(0),
            Constraint::Length(0),
        ])
        .areas(body)
    }

    fn get_lowercase_name(&self, profile: Rc<PokemonEntity>) -> String {
        profile
            .default_name()
            .replace("Galarian form", "galar")
            .replace("Alola Form", "alola")
            .replace("Hisuian form", "hisui")
            .replace("Paldea form", "paldea")
            .replace(" - ", "-")
            .replace(' ', "-")
            .replace('♀', "-f")
            .replace('♂', "-m")
            .to_lowercase()
    }
}

impl StatefulWidget for PokemonProfileWidget {
    type State = PokemonListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Some(profile) = state.profile_with_region_form() else {
            return;
        };

        let region_form_page_num = state.region_form_len();
        let lowercase_name = self.get_lowercase_name(profile.clone());

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

        let (ansi_width, ansi_height, ansi_text) = std::fs::read(
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

        let [name, ansi, iv, ability, navigation] = self.get_render_areas(
            area,
            LayoutParam {
                ansi_height,
                ansi_width,
                show_page_navigation: region_form_page_num > 1,
            },
        );

        AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
            ability,
            buf,
            &mut state.desc_scrollbar_state,
        );

        Overview::new(profile.name.get(), profile.r#type).render(name, buf);
        IVStatus::new(profile.iv).render(iv, buf);

        if let Some(ansi_text) = ansi_text {
            Paragraph::new(ansi_text).render(ansi, buf);
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
                .render(navigation, buf);
        }
    }
}
