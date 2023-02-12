use serde::Deserialize;

use crate::util::TranslateName;

pub static mut DEF_LOCALES: &'static str = "en";

fn get_dict_pm_type(pm_type: &Vec<String>) -> (String, Option<String>) {
    (pm_type[0].clone(), pm_type.get(1).cloned())
}

pub trait DictType {
    fn get_type(&self) -> (String, Option<String>);
}

#[derive(Deserialize, Clone, Copy)]
pub struct PokemonIV {
    pub hp: u16,
    pub att: u16,
    pub def: u16,
    pub s_att: u16,
    pub s_def: u16,
    pub spd: u16,
}

impl Default for PokemonIV {
    fn default() -> Self {
        PokemonIV {
            hp: 0,
            att: 0,
            def: 0,
            s_att: 0,
            s_def: 0,
            spd: 0,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Pokemon {
    pub no: u16,
    pub r#type: Vec<String>,
    pub name: TranslateName,
    pub iv: PokemonIV,
    pub form: Option<Vec<PokemonForm>>,
    pub ability: Vec<u16>,
}

impl Default for Pokemon {
    fn default() -> Self {
        Pokemon {
            name: TranslateName::default(),
            no: 0,
            r#type: vec!["unknown".to_string()],
            iv: PokemonIV::default(),
            form: Option::None,
            ability: vec![],
        }
    }
}

impl DictType for Pokemon {
    fn get_type(&self) -> (String, Option<String>) {
        get_dict_pm_type(&self.r#type)
    }
}

#[derive(Deserialize, Clone)]
pub struct PokemonForm {
    pub form: Vec<String>,
    pub r#type: Vec<String>,
    pub iv: PokemonIV,
    pub ability: Vec<u16>,
}

impl DictType for PokemonForm {
    fn get_type(&self) -> (String, Option<String>) {
        get_dict_pm_type(&self.r#type)
    }
}
