use serde::Deserialize;

use crate::util::TranslateName;

#[derive(Deserialize, Clone, Default)]
pub struct Ability {
    pub no: u16,
    name: TranslateName,
    desc: TranslateName,
}

impl Ability {
    pub fn name(&self) -> String {
        self.name.get_name()
    }

    pub fn desc(&self) -> String {
        self.desc.get_name()
    }
}
