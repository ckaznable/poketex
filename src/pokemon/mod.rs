pub mod pokemon;
pub mod ability;

use std::{rc::Rc, collections::HashMap};

use pokemon::*;
use ability::*;
use serde::Deserialize;

use crate::env::{DEF_LOCALES, LOCALES};

pub type AbilityMap = HashMap<u16, Ability>;

#[derive(Default)]
pub struct PokemonBundle {
    pub pokemon: Rc<Vec<PokemonEntity>>,
    pub ability: Rc<AbilityMap>,
}

impl PokemonBundle {
    fn get_ability(&self, id: Option<u16>) -> Option<PokemonAbilityText> {
        let id = id?;
        let ability = self.ability.get(&id)?;
        Some(PokemonAbilityText {
            name: ability.name().to_string(),
            desc: ability.desc().to_string(),
        })
    }

    pub fn get_ability_text(&self, pm: &PokemonEntity) -> Vec<PokemonAbilityText> {
        let ability = pm.ability();
        let mut list = vec![];

        if let Some(a) = self.get_ability(Some(ability.0)) {
            list.push(a)
        };

        if let Some(a) = self.get_ability(ability.1) {
            list.push(a)
        }

        if let Some(a) = self.get_ability(ability.2) {
            list.push(a)
        }

        if let Some(a) = self.get_ability(ability.3) {
            list.push(a)
        }

        if let Some(a) = self.get_ability(ability.4) {
            list.push(a)
        }

        list
    }
}

pub struct PokemonAbilityText {
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct TranslateText {
    zh: String,
    en: String,
    jp: String,
}

impl<'a> TranslateText {
    pub fn get(&self) -> &'a str {
        unsafe {
            let loc = if !DEF_LOCALES.eq(LOCALES.as_str()) {
                DEF_LOCALES
            } else {
                LOCALES.as_str()
            };

            let text = match loc {
                "en" => self.en.as_str(),
                "zh" => self.zh.as_str(),
                "ja" => self.jp.as_str(),
                _ => self.en.as_str(),
            };

            if !text.is_empty() {
                text
            } else {
                self.en.as_str()
            }
        }
    }
}
