use serde::Deserialize;

use crate::util::TranslateName;

#[derive(Deserialize, Clone)]
pub struct Ability {
    pub no: u16,
    name: TranslateName,
    desc: TranslateName,
}

impl Default for Ability {
    fn default() -> Self {
        Ability {
            no: 0,
            name: TranslateName::default(),
            desc: TranslateName::default(),
        }
    }
}

impl Ability {
    pub fn name(&self) -> String {
        self.name.get_name()
    }

    pub fn desc(&self) -> String {
        self.desc.get_name()
    }
}
