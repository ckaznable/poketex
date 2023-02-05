use tui::widgets::ListState;

use crate::pokemon::{Pokemon, DictType};

use super::dex::{PokemonDexState, PokemonDex};

pub struct PokemonListStatus {
    pub state: ListState,
    pub items: Vec<Pokemon>,
    pub current: Pokemon,
    pub dex: PokemonDexState,
}

fn flat_dex(pm: &Pokemon) -> PokemonDexState {
    let name = pm.name.get_name();
    let mut list = vec![
        PokemonDex {
            name: name.clone(),
            iv: pm.iv,
            pm_type: pm.get_type(),
        }
    ];

    match &pm.form {
        None => (),
        Some(form) => {
            for f in form {
                let name = format!("{} {}", name, f.form.join(" "));
                list.push(
                    PokemonDex {
                        name,
                        iv: f.iv,
                        pm_type: f.get_type()
                    }
                );
            }
        },
    }

    let state = PokemonDexState {
        items: list,
        page: 1,
    };
    state
}

impl PokemonListStatus {
    pub fn new(mut items: Vec<Pokemon>) -> PokemonListStatus {
        // make sure items has def pokemon
        if items.len() == 0 {
            items.push(Pokemon::default());
        };

        // init position = 0
        let mut state = ListState::default();
        state.select(Some(0));

        let current = (&items).get(0).unwrap().clone();

        PokemonListStatus {
            state,
            dex: flat_dex(&current),
            current,
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
        self.current(i);
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
        self.current(i);
    }

    fn current(&mut self, index: usize) {
        self.current = match self.items.get(index) {
            Some(pm) => pm.clone(),
            None => self.items.get(0).unwrap().clone(),
        };
        self.dex = flat_dex(&self.current);
    }
}