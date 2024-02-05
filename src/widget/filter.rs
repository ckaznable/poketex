use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{
    env::LIST_H_MARGIN,
    state::{AppState, InputMode},
};

pub struct Filter;

impl Filter {
    fn paragraph(self, scroll: usize, value: &str) -> Paragraph {
        Paragraph::new(value)
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(Block::bordered())
    }
}

impl StatefulWidget for Filter {
    type State = AppState;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let [layout] = Layout::default()
            .constraints([Constraint::Min(0)])
            .horizontal_margin(LIST_H_MARGIN)
            .areas(area);

        let [wrapper] = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Min(0)])
            .areas(layout);

        Block::default().borders(Borders::LEFT).render(layout, buf);

        match state.tui.input_mode {
            InputMode::Normal => {
                state.tui.cursor = None;
                Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Press '/' search")
                    .render(wrapper, buf);
            }

            InputMode::Editing => {
                let width = area.width.max(3) - 3;
                let scroll = state.key_handle.input.visual_scroll(width as usize);
                self.paragraph(scroll, state.key_handle.input.value())
                    .render(wrapper, buf);
                state.tui.cursor = Some((
                    wrapper.x
                        + ((state.key_handle.input.visual_cursor()).max(scroll) - scroll) as u16
                        + 1,
                    wrapper.y + 1,
                ))
            }
        }
    }
}
