use std::fmt::{self, Display};

use ratatui::style::Color;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

use super::TranslateText;

pub type PokemonAbility = Vec<u16>;
pub type PokemonRegionForm = Vec<PokemonRegionFormEntity>;

pub trait Pokemon {
    fn no(&self) -> u16;
    fn r#type(&self) -> PokemonType;
    fn name(&self) -> String;
    fn iv(&self) -> PokemonIV;
    fn ability(&self) -> PokemonAbility;
}

#[derive(Deserialize, Clone, Copy, Default)]
pub struct PokemonIV {
    pub hp: u16,
    pub att: u16,
    pub def: u16,
    pub s_att: u16,
    pub s_def: u16,
    pub spd: u16,
}

#[derive(Deserialize, Clone, Default)]
pub struct PokemonEntity {
    pub no: u16,
    pub r#type: PokemonType,
    pub name: TranslateText,
    pub iv: PokemonIV,
    pub form: Option<PokemonRegionForm>,
    pub ability: PokemonAbility,
}

impl Pokemon for PokemonEntity {
    #[inline]
    fn no(&self) -> u16 {
        self.no
    }

    #[inline]
    fn r#type(&self) -> PokemonType {
        self.r#type
    }

    #[inline]
    fn name(&self) -> String {
        self.name.get().to_string()
    }

    #[inline]
    fn iv(&self) -> PokemonIV {
        self.iv
    }

    #[inline]
    fn ability(&self) -> PokemonAbility {
        self.ability.clone()
    }
}

impl PokemonEntity {
    fn name_of_list<T: AsRef<str>>(no: T, name: T) -> String {
        format!("#{} {}", no.as_ref(), name.as_ref())
    }

    pub fn name_with_no(&self) -> String {
        PokemonEntity::name_of_list(self.no.to_string(), self.name.get().to_string())
    }

    pub fn region_form(&self) -> Option<Vec<PokemonEntity>> {
        let form = self
            .form
            .clone()?
            .iter()
            .map(|f| PokemonEntity {
                no: self.no,
                r#type: f.r#type,
                name: self.name.clone(),
                ability: f.ability.clone(),
                iv: f.iv,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        Some(form)
    }
}

#[derive(Deserialize, Clone)]
pub struct PokemonRegionFormEntity {
    pub r#type: PokemonType,
    pub iv: PokemonIV,
    pub ability: PokemonAbility,
    pub form: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum PokemonTypeKind {
    Fire,
    Grass,
    Water,
    Normal,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    #[default]
    Other,
}

impl<'de> Deserialize<'de> for PokemonTypeKind {
    fn deserialize<D>(deserializer: D) -> Result<PokemonTypeKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PokemonTypeKindVisitor)
    }
}

impl PokemonTypeKind {
    pub fn color(&self) -> Color {
        use PokemonTypeKind::*;

        match self {
            Fire => Color::Rgb(255, 68, 34),
            Grass => Color::Rgb(119, 204, 85),
            Water => Color::Rgb(51, 153, 255),
            Normal => Color::Rgb(187, 187, 170),
            Electric => Color::Rgb(255, 204, 51),
            Ice => Color::Rgb(119, 221, 255),
            Fighting => Color::Rgb(187, 85, 68),
            Poison => Color::Rgb(170, 85, 153),
            Ground => Color::Rgb(221, 187, 85),
            Flying => Color::Rgb(102, 153, 255),
            Psychic => Color::Rgb(255, 85, 153),
            Bug => Color::Rgb(170, 187, 34),
            Rock => Color::Rgb(187, 170, 102),
            Ghost => Color::Rgb(102, 102, 187),
            Dragon => Color::Rgb(119, 102, 238),
            Dark => Color::Rgb(119, 85, 68),
            Steel => Color::Rgb(170, 170, 187),
            Fairy => Color::Rgb(255, 170, 255),
            _ => Color::Black,
        }
    }
}

impl Display for PokemonTypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PokemonTypeKind::*;

        match self {
            Fire => write!(f, "Fire"),
            Grass => write!(f, "Grass"),
            Water => write!(f, "Water"),
            Normal => write!(f, "Normal"),
            Electric => write!(f, "Electric"),
            Ice => write!(f, "Ice"),
            Fighting => write!(f, "Fighting"),
            Poison => write!(f, "Poison"),
            Ground => write!(f, "Ground"),
            Flying => write!(f, "Flying"),
            Psychic => write!(f, "Psychic"),
            Bug => write!(f, "Bug"),
            Rock => write!(f, "Rock"),
            Ghost => write!(f, "Ghost"),
            Dragon => write!(f, "Dragon"),
            Dark => write!(f, "Dark"),
            Steel => write!(f, "Steel"),
            Fairy => write!(f, "Fairy"),
            _ => write!(f, "Other"),
        }
    }
}

struct PokemonTypeKindVisitor;

impl<'de> Visitor<'de> for PokemonTypeKindVisitor {
    type Value = PokemonTypeKind;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Pokemon Type Kind")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use PokemonTypeKind::*;

        match value {
            "fire" => Ok(Fire),
            "grass" => Ok(Grass),
            "water" => Ok(Water),
            "normal" => Ok(Normal),
            "electric" => Ok(Electric),
            "ice" => Ok(Ice),
            "fighting" => Ok(Fighting),
            "poison" => Ok(Poison),
            "ground" => Ok(Ground),
            "flying" => Ok(Flying),
            "psychic" => Ok(Psychic),
            "bug" => Ok(Bug),
            "rock" => Ok(Rock),
            "ghost" => Ok(Ghost),
            "dragon" => Ok(Dragon),
            "dark" => Ok(Dark),
            "steel" => Ok(Steel),
            "fairy" => Ok(Fairy),
            _ => Ok(Other),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct PokemonType(pub PokemonTypeKind, pub Option<PokemonTypeKind>);

impl<'de> Deserialize<'de> for PokemonType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;

        if let Some(array) = v.as_array() {
            match array.len() {
                1 => {
                    let first = serde_json::from_value(array[0].clone()).expect("wrong type");
                    Ok(PokemonType(first, None))
                }
                2 => {
                    let first = serde_json::from_value(array[0].clone()).expect("wrong type");
                    let second = serde_json::from_value(array[1].clone()).expect("wrong type");
                    Ok(PokemonType(first, Some(second)))
                }
                _ => Err(serde::de::Error::invalid_length(
                    array.len(),
                    &"array with one or two elements",
                )),
            }
        } else {
            Err(serde::de::Error::custom("expected an array"))
        }
    }
}
