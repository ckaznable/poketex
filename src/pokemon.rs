use crate::env::LOCALES;
use serde::Deserialize;

pub static mut DEF_LOCALES: &'static str = "en";

fn get_dict_pm_type(pm_type: &Vec<String>) -> (String, Option<String>) {
    (pm_type[0].clone(), pm_type.get(1).cloned())
}

pub trait DictType {
    fn get_type(&self) -> (String, Option<String>);
}

#[derive(Deserialize, Clone)]
pub struct PokemonName {
    pub zh: String,
    pub en: String,
    pub jp: String,
}

impl PokemonName {
    pub fn get_name(&self) -> String {
        let sp: Vec<&str> = LOCALES.as_str().split("-").collect();

        unsafe {
            let loc = if !DEF_LOCALES.eq("en") {
                DEF_LOCALES
            } else {
                *sp.get(0).unwrap()
            };

            match loc {
                "en" => self.en.to_owned(),
                "zh" => self.zh.to_owned(),
                "ja" => self.jp.to_owned(),
                _ => self.en.to_owned(),
            }
        }
    }
}

impl Default for PokemonName {
    fn default() -> Self {
        PokemonName {
            zh: "".to_string(),
            en: "".to_string(),
            jp: "".to_string(),
        }
    }
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
    pub name: PokemonName,
    pub iv: PokemonIV,
    pub form: Option<Vec<PokemonForm>>,
}

impl Default for Pokemon {
    fn default() -> Self {
        Pokemon {
            name: PokemonName::default(),
            no: 0,
            r#type: vec!["unknown".to_string()],
            iv: PokemonIV::default(),
            form: Option::None,
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
}

impl DictType for PokemonForm {
    fn get_type(&self) -> (String, Option<String>) {
        get_dict_pm_type(&self.r#type)
    }
}
