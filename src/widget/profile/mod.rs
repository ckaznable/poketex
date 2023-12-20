mod ability;
mod iv;
mod overview;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, StatefulWidget, Widget},
};
use ratatui_image::ResizeImage;

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

        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(5),
                Constraint::Length(11),
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ],
        )
        .split(area);

        Overview::new(profile.name.get(), profile.r#type).render(layout[0], buf);

        let image_index = profile.no.saturating_sub(1) as usize;
        if state.pokemon_images.get(image_index).is_some() {
            ResizeImage::new(None).render(layout[2], buf, &mut state.pokemon_images[image_index]);
        }

        IVStatus::new(profile.iv).render(layout[3], buf);

        AbilityParaGraph(state.bundle.get_ability_text(&profile)).render(
            layout[5],
            buf,
            &mut state.desc_scrollbar_state,
        );

        let page_num = state.region_form_len();
        if page_num > 1 {
            let title = format!("<- {} / {} ->", state.profile_page + 1, page_num,);

            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .render(layout[6], buf);
        }
    }
}
