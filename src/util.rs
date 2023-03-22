use ratatui::style::Color;
use serde::Deserialize;

use crate::env::{DEF_LOCALES, LOCALES};

#[derive(Deserialize, Clone)]
pub struct TranslateName {
    pub zh: String,
    pub en: String,
    pub jp: String,
}

impl TranslateName {
    pub fn get_name(&self) -> String {
        let sp: Vec<&str> = LOCALES.as_str().split("-").collect();
        let env_locales = *sp.get(0).unwrap();

        unsafe {
            let loc = if !DEF_LOCALES.eq(env_locales) {
                DEF_LOCALES
            } else {
                env_locales
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

impl Default for TranslateName {
    fn default() -> Self {
        TranslateName {
            zh: "".to_string(),
            en: "".to_string(),
            jp: "".to_string(),
        }
    }
}

pub fn get_type_bg_color(t: &str) -> Color {
    match t {
        "fire" => Color::Rgb(255, 68, 34),
        "grass" => Color::Rgb(119, 204, 85),
        "water" => Color::Rgb(51, 153, 255),
        "normal" => Color::Rgb(187, 187, 170),
        "electric" => Color::Rgb(255, 204, 51),
        "ice" => Color::Rgb(119, 221, 255),
        "fighting" => Color::Rgb(187, 85, 68),
        "poison" => Color::Rgb(170, 85, 153),
        "ground" => Color::Rgb(221, 187, 85),
        "flying" => Color::Rgb(102, 153, 255),
        "psychic" => Color::Rgb(255, 85, 153),
        "bug" => Color::Rgb(170, 187, 34),
        "rock" => Color::Rgb(187, 170, 102),
        "ghost" => Color::Rgb(102, 102, 187),
        "dragon" => Color::Rgb(119, 102, 238),
        "dark" => Color::Rgb(119, 85, 68),
        "steel" => Color::Rgb(170, 170, 187),
        "fairy" => Color::Rgb(255, 170, 255),
        _ => Color::Black,
    }
}
