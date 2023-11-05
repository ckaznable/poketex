use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{constant::LIST_H_MARGIN, state::{AppState, InputMode}};

pub struct Filter<'a>(pub &'a AppState<'a>);

impl<'a> Filter<'a> {
    fn paragraph(self, scroll: usize, value: &str) -> Paragraph {
        Paragraph::new(value)
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL))
    }
}

impl<'a> Widget for Filter<'a> {
    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
    ) {
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

        match self.0.tui.input_mode {
            InputMode::Normal => {
                self.0.tui.cursor = None;
                Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Press '/' search")
                    .render(wrapper[0], buf);
            }

            InputMode::Editing => {
                let width = area.width.max(3) - 3;
                let scroll = self.0.key_handle.input.visual_scroll(width as usize);
                self.paragraph(scroll, self.0.key_handle.input.value()).render(wrapper[0], buf);
                self.0.tui.cursor = Some((
                    wrapper[0].x + ((self.0.key_handle.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    wrapper[0].y + 1,
                ))
            }
        }
    }
}
