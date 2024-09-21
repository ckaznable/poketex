use super::InputMode;

#[derive(Default, Copy, Clone)]
pub struct TuiState {
    pub show_help: bool,
    pub show_list: bool,
    pub show_abilities: bool,
    pub show_iv: bool,
    pub cursor: Option<(u16, u16)>,
    pub input_mode: InputMode,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            show_list: true,
            ..Default::default()
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_show_list(&mut self) {
        self.show_list = !self.show_list;
    }

    pub fn toggle_show_abilities(&mut self) {
        self.show_abilities = !self.show_abilities;
    }
}
