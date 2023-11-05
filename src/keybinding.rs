use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::state::{AppState, InputMode};

static PAGE_NUM: u8 = 4;

#[derive(Default, Eq, PartialEq)]
pub enum KeyHandleResult {
    #[default]
    Continue,
    Exit,
}

impl KeyHandleResult {
    pub fn is_exit(&self) -> bool {
        *self == KeyHandleResult::Exit
    }
}

fn on_editing(app: &mut AppState, event: KeyEvent) -> KeyHandleResult {
    use KeyCode::*;

    match event.code {
        Esc => {
            app.reset();
            app.pokemon_list.filter_query.clear();
        }
        _ => {
            app.key_handle.input.handle_event(&Event::Key(event));
            app.pokemon_list.filter_query.clear();
            app.pokemon_list.filter_query.push_str(app.key_handle.input.value());
        }
    };

    KeyHandleResult::Continue
}

fn on_normal(app: &mut AppState, event: KeyEvent) -> KeyHandleResult {
    use KeyCode::*;

    match event {
        // handle key with control
        KeyEvent {
            code: c,
            modifiers: KeyModifiers::CONTROL,
            kind: _,
            state: _,
        } => {
            match c {
                Char('f') => app.pokemon_list.scroll_down(PAGE_NUM),
                Char('b') => app.pokemon_list.scroll_up(PAGE_NUM),
                _ => return KeyHandleResult::Continue
            }
            // app.cancel_last_cmd();
        }

        // handle number key
        KeyEvent {
            code: Char(c),
            modifiers: _,
            kind: _,
            state: _,
        } if c.is_ascii_digit() => {
            app.pokemon_list.filter_query.push(c);
            // app.go_top(false);
        }

        // handle other key
        KeyEvent {
            code: c,
            modifiers: _,
            kind: _,
            state: _,
        } => {
            match c {
                Char('q') => return KeyHandleResult::Exit,

                Down | Char('j') => {
                    if app.pokemon_list.is_scroll_tail() {
                        app.pokemon_list.list_scrollbar_state.first();
                    } else {
                        app.pokemon_list.list_scrollbar_state.next();
                    }

                    app.pokemon_list.next();
                }
                PageDown => {
                    app.pokemon_list.scroll_down(PAGE_NUM);
                    (0..PAGE_NUM).for_each(|_| app.pokemon_list.list_scrollbar_state.next());
                }

                Up | Char('k') => {
                    if app.pokemon_list.is_scroll_head() {
                        app.pokemon_list.list_scrollbar_state.last();
                    } else {
                        app.pokemon_list.list_scrollbar_state.prev();
                    }

                    app.pokemon_list.previous();
                }
                PageUp => {
                    app.pokemon_list.scroll_up(PAGE_NUM);
                    (0..PAGE_NUM).for_each(|_| app.pokemon_list.list_scrollbar_state.prev());
                }

                Left | Char('h') => app.pokemon_list.previous_profile_page(),
                Right | Char('l') => app.pokemon_list.next_profile_page(),

                Char('/') => app.tui.input_mode = InputMode::Editing,
                Esc => app.tui.query.clear(),

                Home => app.jump(1),
                End => app.jump(app.pokemon_list.len()),

                Char('g') => {
                    // if app.go_top {
                    //     app.jump(1);
                    //     app.pokemon_list.list_scrollbar_state.first();
                    //     app.go_top(false);
                    // } else {
                    //     app.go_top(true);
                    // }
                }
                Char('G') => {
                    // if app.no.eq("") {
                    //     app.jump(app.pokemon_list.len());
                    //     app.pokemon_list.list_scrollbar_state.last();
                    // } else {
                    //     let index = app.no.trim().parse::<usize>().unwrap();
                    //     app.jump(index);
                    //     app.pokemon_list.list_scrollbar_state.position(index);
                    // }

                    // app.no(String::from(""));
                    // app.go_top(false);
                }

                Char('H') => {
                    app.toggle_help();
                }

                _ => {}
            }

            // app.cancel_last_cmd();
        }
    }

    KeyHandleResult::Continue
}

pub fn handle_key(app: &mut AppState, event: KeyEvent) -> KeyHandleResult {
    use KeyHandleResult::*;

    if event.kind == KeyEventKind::Release {
        return Continue;
    }

    match app.tui.input_mode {
        InputMode::Editing => on_editing(app, event),
        InputMode::Normal => on_normal(app, event)
    }
}
