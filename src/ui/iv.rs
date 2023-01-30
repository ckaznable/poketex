use tui::{layout::{Layout, Direction, Constraint, Rect}, widgets::{Block, Gauge, Borders}, style::{Style, Color, Modifier}, backend::Backend, Frame};

pub fn set_iv_layout<'a, B: Backend>(title: &'a str, iv: &u16, max_iv: f32, area: Rect, f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(60), Constraint::Min(0)])
        .split(area);

    let block = Block::default().title(title);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(((*iv as f32 / max_iv) * 100.0) as u16)
        .label(iv.to_string());

    f.render_widget(block, chunks[0]);
    f.render_widget(gauge, chunks[1]);
}