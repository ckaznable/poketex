use tui::style::Color;

use crate::pokemon::PokemonIV;

pub fn get_pokemon_iv_highest(pm: &PokemonIV) -> u16 {
    *vec![pm.hp, pm.att, pm.def, pm.s_att, pm.s_def, pm.spd].iter().max().unwrap()
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
        _ => Color::Black
    }
}