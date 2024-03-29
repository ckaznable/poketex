use super::InputMode;

#[derive(Default)]
pub struct TuiState {
    pub show_help: bool,
    pub cursor: Option<(u16, u16)>,
    pub input_mode: InputMode,
}
