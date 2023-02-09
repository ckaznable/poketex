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
    fn paragraph<'a>(self, scroll: usize, state: &'a mut AppState) -> Paragraph<'a> {
        Paragraph::new(state.input.value())
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL))
    }
}

impl StatefulWidget for Filter {
    type State = AppState;

    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([Constraint::Min(0)])
            .horizontal_margin(LIST_H_MARGIN)
            .split(area);

        let wrapper = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Min(0)])
            .split(layout[0]);

        Block::default()
            .borders(Borders::LEFT)
            .render(layout[0], buf);

        match state.input_mode {
            InputMode::Normal => {
                Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Press '/' search")
                    .render(wrapper[0], buf);
            }

            InputMode::Editing => {
                let width = area.width.max(3) - 3;
                let scroll = state.input.visual_scroll(width as usize);
                self.paragraph(scroll, state).render(wrapper[0], buf);
            }
        }
    }
}