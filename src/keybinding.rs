use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
        Esc => app.reset(),
        Enter => app.tui.input_mode = InputMode::Normal,
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

    let KeyEvent {
        code,
        modifiers,
        kind: _,
        state: _,
    } = event;

    match (code, modifiers) {
        // handle key with control
        (c, KeyModifiers::CONTROL) => match c {
            Char('f') => app.pokemon_list.scroll_down(PAGE_NUM),
            Char('b') => app.pokemon_list.scroll_up(PAGE_NUM),
            _ => return KeyHandleResult::Continue,
        },

        // handle key with alt
        (c, KeyModifiers::ALT) => match c {
            Char('j') => app.pokemon_list.desc_scrollbar_state.scroll_down(),
            Char('k') => app.pokemon_list.desc_scrollbar_state.scroll_up(),
            _ => (),
        },

        // handle other key
        (c, _) => match c {
            Char('q') => return KeyHandleResult::Exit,
            Char('H') => app.tui.toggle_help(),
            Char('E') => app.tui.toggle_show_list(),
            Char('A') => app.tui.toggle_show_abilities(),

            Down | Char('j') => app.pokemon_list.next(),
            PageDown => app.pokemon_list.scroll_down(PAGE_NUM),

            Up | Char('k') => app.pokemon_list.previous(),
            PageUp => app.pokemon_list.scroll_up(PAGE_NUM),

            Left | Char('h') => app.pokemon_list.previous_profile_page(),
            Right | Char('l') => app.pokemon_list.next_profile_page(),

            Char('f') => app.pokemon_list.increase_ascii_form_index(),
            Char('/') => app.tui.input_mode = InputMode::Editing,

            Home => app.pokemon_list.scroll_to_first(),
            End => app.pokemon_list.scroll_to_end(),

            Char(c) => app.command(c),
            Enter | Esc => app.reset_command(),
            _ => (),
        },
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
