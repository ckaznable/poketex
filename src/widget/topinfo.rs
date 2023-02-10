use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Widget},
};

use crate::util::get_type_bg_color;

pub struct TopInfo {
    name: String,
    pm_type: (String, Option<String>),
}

impl TopInfo {
    pub fn new(name: String, pm_type: (String, Option<String>)) -> Self {
        TopInfo { name, pm_type }
    }
}

impl Widget for TopInfo {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let mut type_span = vec![
            Span::from(String::from(self.name) + " "),
            Span::styled(
                self.pm_type.0.clone(),
                Style::default()
                    .bg(get_type_bg_color(self.pm_type.0.as_str()))
                    .fg(Color::White),
            ),
            Span::from(" "),
        ];

        match self.pm_type.1 {
            Some(t) => {
                if !t.eq("unknow") {
                    type_span.push(Span::styled(
                        t.clone(),
                        Style::default()
                            .bg(get_type_bg_color(t.as_str()))
                            .fg(Color::White),
                    ));
                }
                ()
            }
            None => (),
        };

        Block::default()
            .title(type_span)
            .borders(Borders::NONE)
            .render(layout[0], buf);
    }
}
