pub mod ability;
pub mod ascii_form;
#[allow(clippy::module_inception)]
pub mod pokemon;
mod translate;

use std::{collections::HashMap, rc::Rc};

use ability::*;
pub use pokemon::*;
pub use translate::*;

pub type AbilityMap = HashMap<u16, Ability>;

#[derive(Default)]
pub struct PokemonBundle {
    pub pokemon: Vec<Rc<PokemonEntity>>,
    pub ability: Rc<AbilityMap>,
}

impl PokemonBundle {
    fn get_ability(&self, id: u16) -> Option<PokemonAbilityText> {
        let ability = self.ability.get(&id)?;
        Some(PokemonAbilityText {
            name: ability.name().to_string(),
            desc: ability.desc().to_string(),
        })
    }

    pub fn get_ability_text(&self, pm: &PokemonEntity) -> Vec<PokemonAbilityText> {
        pm.ability()
            .iter()
            .filter_map(|id| self.get_ability(*id))
            .collect::<Vec<_>>()
    }
}

pub struct PokemonAbilityText {
    pub name: String,
    pub desc: String,
}
