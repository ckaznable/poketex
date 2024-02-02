pub mod pokemon;
pub mod tui;

pub use pokemon::PokemonListState;
use regex::Regex;

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
pub struct AppState {
    pub tui: TuiState,
    pub key_handle: KeyHandleState,
    pub pokemon_list: PokemonListState,
    pub vim_cmd: String,
}

impl AppState {
    pub fn reset(&mut self) {
        self.tui.input_mode = InputMode::Normal;
        self.key_handle.input.reset();
        self.pokemon_list.set_list_filter(String::from(""))
    }

    pub fn jump(&mut self, i: usize) {
        if i > 0 || i - 1 > self.pokemon_list.len() {
            self.pokemon_list.select(i.saturating_sub(1));
        }
    }

    pub fn toggle_help(&mut self) {
        self.tui.show_help = !self.tui.show_help;
    }

    pub fn command(&mut self, cmd: char) {
        self.vim_cmd.push(cmd);

        match self.vim_cmd.as_str() {
            "gg" => {
                self.pokemon_list.scroll_to_first();
                self.reset_command()
            }
            "G" => {
                self.pokemon_list.scroll_to_end();
                self.reset_command()
            }
            s => {
                if let Some(n) = Regex::new(r"(\d+)G").unwrap().captures(s) {
                    if let Ok(n) = n.get(1).unwrap().as_str().parse::<usize>() {
                        self.pokemon_list.select(n.saturating_sub(1));
                    }

                    self.reset_command()
                }
            }
        }
    }

    pub fn reset_command(&mut self) {
        self.vim_cmd.clear()
    }
}
