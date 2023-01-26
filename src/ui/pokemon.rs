use tui::widgets::ListState;

use crate::pokemon::{Pokemon, PokemonName, PokemonIV};

pub struct PokemonList {
    pub state: ListState,
    pub items: Vec<Pokemon>,
}

impl PokemonList {
    pub fn new(mut items: Vec<Pokemon>) -> PokemonList {
        // make sure items has def pokemon
        if items.len() == 0 {
            let pm = Pokemon {
                name: PokemonName { zh: "".to_string(), en: "".to_string(), jp: "".to_string() },
                no: 0,
                r#type: vec!["unknow".to_string()],
                iv: PokemonIV {
                    hp: 0,
                    att: 0,
                    def: 0,
                    s_att: 0,
                    s_def: 0,
                    spd: 0
                }
            };

            items.push(pm);
        };

        // init position = 0
        let mut state = ListState::default();
        state.select(Some(0));

        PokemonList {
            state,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn get_index(&self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn get_current_item(&self) -> &Pokemon {
        let i = self.get_index();
        match self.items.get(i) {
            Some(pm) => pm,
            None => self.items.get(0).unwrap(),
        }
    }
}