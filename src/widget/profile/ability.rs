use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget, Wrap, StatefulWidget, Scrollbar, ScrollbarOrientation}, buffer::Buffer, style::{Color, Style},
};

use crate::{pokemon::PokemonAbilityText, state::scroll::ScrollableParagraphState};

pub struct AbilityParaGraph(pub Vec<PokemonAbilityText>);

impl StatefulWidget for AbilityParaGraph {
    type State = ScrollableParagraphState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State,) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let (span, count) = self.0.iter().enumerate().fold((vec![], 0), |(mut line, mut count), (i, a)| {
            if i > 0 {
                line.push(Line::from(""));
                count += 1;
            }

            line.push(Line::from(a.name.clone()));
            count += 1;

            let desc = get_lines(&a.desc, layout[0].width as usize - 2);
            count += desc.len();
            desc.into_iter().for_each(|x| line.push(Line::from(x)));

            (line, count)
        });

        let layout_height = layout[0].height as usize;
        state.set_height(if count > layout_height.saturating_sub(2) { count } else { 0 });

        Paragraph::new(span)
            .block(Block::default().title("Ability").borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .scroll((state.position as u16, 0))
            .render(layout[0], buf);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().bg(Color::DarkGray))
            .render(layout[0], buf, &mut state.scrollbar_state);
    }
}

fn get_lines(text: &str, width: usize) -> Vec<String> {
    let options = textwrap::Options::new(width)
        .word_separator(textwrap::WordSeparator::AsciiSpace);

    let lines = textwrap::wrap(text, &options);
    lines.into_iter().map(|x| x.to_string()).collect()
}
