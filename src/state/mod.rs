pub mod tui;
pub mod scroll;

pub use scroll::PokemonListState;

use tui_input::Input;

use self::tui::TuiState;

#[derive(Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct KeyHandleState {
    pub input: Input,
}

#[derive(Default)]
pub struct AppState<'a> {
    pub tui: TuiState,
    pub key_handle: KeyHandleState,
    pub pokemon_list: PokemonListState<'a>,
}

impl<'a> AppState<'a> {
    pub fn reset(&mut self) {
        self.tui.input_mode = InputMode::Normal;
        self.key_handle.input.reset();
    }

    pub fn jump(&mut self, i: usize) {
        if i > 0 || i - 1 > self.pokemon_list.len() {
            self.pokemon_list.select(i.saturating_sub(1));
        }
    }

    pub fn toggle_help(&mut self) {
        self.tui.show_help = !self.tui.show_help;
    }
}
