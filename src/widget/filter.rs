use tui::{
    layout::{Constraint, Layout, Alignment},
    widgets::{StatefulWidget, Block, Borders, Paragraph, Widget}, style::{Color, Style},
};

use crate::{InputMode, constant::LIST_H_MARGIN, AppState};

pub struct Filter {}

impl Default for Filter {
    fn default() -> Self {
        Filter {}
    }
}

impl Filter {
    fn paragraph<'a>(self, area: &tui::layout::Rect, state: &'a mut AppState) -> Paragraph<'a> {
        let width = area.width.max(3) - 3;
        let scroll = state.input.visual_scroll(width as usize);
        Paragraph::new(state.input.value())
            .style(match state.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL))
    }
}

impl StatefulWidget for Filter {
    type State = AppState;

    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .horizontal_margin(LIST_H_MARGIN)
            .split(area);

        match state.input_mode {
            InputMode::Normal => {
                Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Press '/' search")
                    .render(layout[1], buf);
            }

            InputMode::Editing => {
                self.paragraph(&layout[0], state).render(layout[1], buf);
            }
        }
    }
}