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
        Enter => {
            app.tui.input_mode = InputMode::Normal;
        }
        _ => {
            app.key_handle.input.handle_event(&Event::Key(event));
            app.pokemon_list.filter_query.clear();
            app.pokemon_list
                .set_list_filter(app.key_handle.input.value().to_string());
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
                _ => return KeyHandleResult::Continue,
            }
            // app.cancel_last_cmd();
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

                Down | Char('j') => app.pokemon_list.next(),
                PageDown => app.pokemon_list.scroll_down(PAGE_NUM),

                Up | Char('k') => app.pokemon_list.previous(),
                PageUp => app.pokemon_list.scroll_up(PAGE_NUM),

                Left | Char('h') => app.pokemon_list.previous_profile_page(),
                Right | Char('l') => app.pokemon_list.next_profile_page(),

                Char('/') => app.tui.input_mode = InputMode::Editing,

                Home => app.pokemon_list.scroll_to_first(),
                End => app.pokemon_list.scroll_to_end(),

                Char('H') => {
                    app.toggle_help();
                }

                Char(c) => app.command(c),
                Enter|Esc => app.reset_command(),
                _ => {}
            }
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
        InputMode::Normal => on_normal(app, event),
    }
}
