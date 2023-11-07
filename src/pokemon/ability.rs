use serde::Deserialize;

use super::TranslateText;

#[derive(Deserialize, Clone, Default)]
pub struct Ability {
    pub no: u16,
    name: TranslateText,
    desc: TranslateText,
}

impl Ability {
    pub fn name(&self) -> String {
        self.name.get()
    }

    pub fn desc(&self) -> String {
        self.desc.get()
    }
}
