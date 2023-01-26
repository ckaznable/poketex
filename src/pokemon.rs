use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonName {
    pub zh: String,
    pub en: String,
    pub jp: String,
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