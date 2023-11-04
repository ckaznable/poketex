use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::{AppState, InputMode};

static PAGE_NUM: u8 = 4;

#[derive(Default, Eq, PartialEq)]
pub enum KeyHandleResult {
    #[default]
    Continue,
    Exit,
}

impl KeyHandleResult {
    pub fn is_exit(&self) -> bool {
        matches!(self, KeyHandleResult::Exit)
    }
}

pub fn handle_key(app: &mut AppState, event: KeyEvent) -> KeyHandleResult {
    use KeyHandleResult::*;

    if event.kind == KeyEventKind::Release {
        return Continue;
    }

    // in edit mode
    if let InputMode::Editing = app.input_mode {
        match event.code {
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
        };

        return Continue;
    };
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
                    return Continue;
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
        } if c.is_ascii_digit() => {
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
                KeyCode::Char('q') => return Exit,

                KeyCode::Down | KeyCode::Char('j') => {
                    if app.pm.is_current_tail() {
                        app.list_scrollbar_state.first();
                    } else {
                        app.list_scrollbar_state.next();
                    }

                    app.pm.next();
                }
                KeyCode::PageDown => {
                    app.pm.scroll_down(PAGE_NUM);
                    (0..PAGE_NUM).for_each(|_| app.list_scrollbar_state.next());
                }

                KeyCode::Up | KeyCode::Char('k') => {
                    if app.pm.is_current_head() {
                        app.list_scrollbar_state.last();
                    } else {
                        app.list_scrollbar_state.prev();
                    }

                    app.pm.previous();
                }
                KeyCode::PageUp => {
                    app.pm.scroll_up(PAGE_NUM);
                    (0..PAGE_NUM).for_each(|_| app.list_scrollbar_state.prev());
                }

                KeyCode::Left | KeyCode::Char('h') => app.pm.dex.previous(),
                KeyCode::Right | KeyCode::Char('l') => app.pm.dex.next(),

                KeyCode::Char('/') => app.input_mode = InputMode::Editing,
                KeyCode::Esc => app.query(String::from("")),

                KeyCode::Home => app.jump(1),
                KeyCode::End => app.jump(app.pm.items.len()),

                KeyCode::Char('g') => {
                    if app.go_top {
                        app.jump(1);
                        app.list_scrollbar_state.first();
                        app.go_top(false);
                    } else {
                        app.go_top(true);
                    }
                }
                KeyCode::Char('G') => {
                    if app.no.eq("") {
                        app.jump(app.pm.items.len());
                        app.list_scrollbar_state.last();
                    } else {
                        let index = app.no.trim().parse::<usize>().unwrap();
                        app.jump(index);
                        app.list_scrollbar_state.position(index);
                    }

                    app.no(String::from(""));
                    app.go_top(false);
                }

                KeyCode::Char('H') => {
                    app.toggle_help();
                }

                _ => {}
            }

            app.cancel_last_cmd();
        }
    };

    Continue
}
