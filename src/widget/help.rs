use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph, Widget},
};

pub struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let block = Block::bordered()
            .style(Style::default().bg(Color::Gray))
            .title_alignment(Alignment::Center)
            .title("Help");

        let text = "
Use arrow keys or hjkl to move\n
Press page up or page down to move 4 lines at a time\n
Press f to switch ascii forms\n
Press q to exit\n
Press H for help\n
Press gg to go to the top and G to go to the bottom\n
Press alt+j or alt+k to scroll up or down ability description\n
Use / to enter search mode\n\n\n\n
Press H to exit help";

        Paragraph::new(text)
            .style(Style::default().fg(Color::Black))
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buf);
    }
}
