use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::{AppState, InputMode};

static PAGE_NUM: u8 = 4;

pub fn handle_key(mut app: &mut AppState, event: KeyEvent) {
    match event {
        // handle key with control
        KeyEvent {
            code: c,
            modifiers: KeyModifiers::CONTROL,
            kind: _,
            state: _,
        } => {
            match c {
                KeyCode::Char('f') => app.pm.scroll_down(PAGE_NUM),
                KeyCode::Char('b') => app.pm.scroll_up(PAGE_NUM),
                _ => {
                    return;
                }
            }
            app.cancel_last_cmd();
        }

        // handle number key
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: _,
            kind: _,
            state: _,
        } if ('0'..='9').contains(&c) => {
            app.no(app.no.clone() + &c.to_string());
            app.go_top(false);
        }

        // handle other key
        KeyEvent {
            code: c,
            modifiers: _,
            kind: _,
            state: _,
        } => {
            match c {
                KeyCode::Char('q') => return,

                KeyCode::Down => app.pm.next(),
                KeyCode::Char('j') => app.pm.next(),
                KeyCode::PageDown => app.pm.scroll_down(PAGE_NUM),

                KeyCode::Up => app.pm.previous(),
                KeyCode::Char('k') => app.pm.previous(),
                KeyCode::PageUp => app.pm.scroll_up(PAGE_NUM),

                KeyCode::Left => app.pm.dex.previous(),
                KeyCode::Char('h') => app.pm.dex.previous(),

                KeyCode::Right => app.pm.dex.next(),
                KeyCode::Char('l') => app.pm.dex.next(),

                KeyCode::Char('/') => app.input_mode = InputMode::Editing,
                KeyCode::Esc => app.query(String::from("")),

                KeyCode::Home => app.jump(1),
                KeyCode::End => app.jump(app.pm.items.len()),

                KeyCode::Char('g') => {
                    if app.go_top {
                        app.jump(1);
                        app.go_top(false);
                    } else {
                        app.go_top(true);
                    }
                }
                KeyCode::Char('G') => {
                    if app.no.eq("") {
                        app.jump(app.pm.items.len());
                    } else {
                        let index = app.no.trim().parse::<usize>().unwrap();
                        app.jump(index);
                    }

                    app.no(String::from(""));
                    app.go_top(false);
                }

                _ => {}
            }

            app.cancel_last_cmd();
        }
    };

    match app.input_mode {
        InputMode::Editing => match event.code {
            KeyCode::Enter => {
                app.query(app.input.value().to_owned());
                app.reset();
            }
            KeyCode::Esc => {
                app.reset();
                app.query(String::from(""));
            }
            _ => {
                app.input.handle_event(&Event::Key(event));
                app.query(app.input.value().to_owned());
            }
        },
        _ => {}
    };
}
