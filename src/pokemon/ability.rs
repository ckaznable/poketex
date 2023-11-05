use serde::Deserialize;

use super::TranslateText;

#[derive(Deserialize, Clone, Default)]
pub struct Ability {
    pub no: u16,
    name: TranslateText,
    desc: TranslateText,
}

impl<'a> Ability {
    pub fn name(&self) -> &'a str {
        self.name.get()
    }

    pub fn desc(&self) -> &'a str {
        self.desc.get()
    }
}
