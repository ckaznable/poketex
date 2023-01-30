use serde::{Deserialize, Serialize};
use crate::env::LOCALES;

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonName {
    pub zh: String,
    pub en: String,
    pub jp: String,
}

impl PokemonName {
    pub fn get_name(&self) -> String {
        let sp: Vec<&str> = LOCALES.as_str().split("-").collect();

        match *sp.get(0).unwrap() {
            "en" => self.en.to_owned(),
            "zh" => self.zh.to_owned(),
            "ja" => self.jp.to_owned(),
            _ => self.en.to_owned()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonIV {
    pub hp: u16,
    pub att: u16,
    pub def: u16,
    pub s_att: u16,
    pub s_def: u16,
    pub spd: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub no: u16,
    pub r#type: Vec<String>,
    pub name: PokemonName,
    pub iv: PokemonIV,
}