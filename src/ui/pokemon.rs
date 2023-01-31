use tui::widgets::ListState;

use crate::pokemon::Pokemon;

pub struct PokemonDex {
    pub state: ListState,
    pub items: Vec<Pokemon>,
}

impl PokemonDex {
    pub fn new(mut items: Vec<Pokemon>) -> PokemonDex {
        // make sure items has def pokemon
        if items.len() == 0 {
            items.push(Pokemon::default());
        };

        // init position = 0
        let mut state = ListState::default();
        state.select(Some(0));

        PokemonDex {
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