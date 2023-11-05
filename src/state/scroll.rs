use std::rc::Rc;

use ratatui::widgets::{ScrollbarState, ListState};

use crate::pokemon::{PokemonBundle, pokemon::PokemonEntity, AbilityMap};

#[derive(Default)]
pub struct PokemonListState<'a> {
    pub list_scrollbar_state: ScrollbarState,
    pub list_state: ListState,
    pub list_item: &'a [PokemonEntity],
    pub filter_query: String,
    pub bundle: Rc<PokemonBundle>,
    pub profile_page: u8,
    pub profile: Option<&'a PokemonEntity>,
}

impl<'a> PokemonListState<'a> {
    pub fn pokemons(&self) -> Rc<Vec<PokemonEntity>> {
        self.bundle.pokemon
    }

    pub fn len(&self) -> usize {
        self.bundle.pokemon.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bundle.pokemon.is_empty()
    }

    pub fn ability_map(&self) -> Rc<AbilityMap> {
        self.bundle.ability
    }

    pub fn scroll_length(mut self, len: usize) -> Self {
        self.list_scrollbar_state = self.list_scrollbar_state.content_length(len);
        self
    }

    pub fn next(&mut self) {
        let index = match self.list_state.selected() {
            Some(i) => {
                if i >= self.len() - 1 {
                    0
                } else {
                    i.saturating_add(1)
                }
            }
            None => 0,
        };
        self.select(index);
    }

    pub fn previous(&mut self) {
        let index = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    if !self.is_empty() {
                        self.len() - 1
                    } else {
                        i
                    }
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        self.select(index);
    }

    pub fn scroll_down(&mut self, amount: u8) {
        if let Some(i) = self
            .list_state
            .selected()
            .and_then(|v| v.checked_add(amount.into()))
            .map(|mut index| {
                if index > self.len() {
                    index = self.len() - 1;
                }
                index
            })
        {
            self.select(i);
        }
    }

    pub fn scroll_up(&mut self, amount: u8) {
        if let Some(i) = self
            .list_state
            .selected()
            .and_then(|v| v.checked_sub(amount.into()))
            .or(Some(0))
        {
            self.select(i);
        }
    }

    pub fn set_list_filter(&mut self, filter: String) {
        self.filter_query = filter;

        self.list_item = if filter == "" {
            self.bundle.pokemon.as_slice()
        } else {
            self.bundle
                .pokemon
                .iter()
                .filter(|item| item
                    .name_with_no()
                    .to_lowercase()
                    .contains(filter.to_lowercase().as_str())
                )
                .cloned()
                .collect::<Vec<_>>()
                .as_slice()
        };

        self.select(0);
    }

    pub fn select(&mut self, index: usize) {
        self.list_state.select(Some(index));
    }

    pub fn is_scroll_head(&self) -> bool {
        if let Some(i) = self.list_state.selected() {
            i == 0
        } else {
            false
        }
    }

    pub fn is_scroll_tail(&self) -> bool {
        if let Some(i) = self.list_state.selected() {
            i == self.len() - 1
        } else {
            false
        }
    }

    pub fn profile(&self) -> Option<&'a PokemonEntity> {
        self.pokemons().get(self.list_state.selected()?)
    }

    pub fn profile_with_region_form(&self) -> Option<&'a PokemonEntity> {
        let profile = self.profile()?;
        if self.profile_page > 0 {
            Some(profile.region_form()?.get((self.profile_page as usize).saturating_sub(1))?)
        } else {
            Some(profile)
        }
    }

    pub fn region_form_len(&self) -> u8 {
        let Some(profile) = self.profile() else {
            return 0;
        };

        let Some(profile) = profile.region_form() else {
            return 0;
        };

        profile.len().saturating_add(1) as u8
    }

    pub fn next_profile_page(&mut self) {
        let len = self.region_form_len();
        if len > 0 && self.profile_page < len - 1 {
            self.profile_page = self.profile_page.saturating_add(1);
        }
    }

    pub fn previous_profile_page(&mut self) {
        let len = self.region_form_len();
        if len > 0 && self.profile_page > 0 {
            self.profile_page = self.profile_page.saturating_sub(1);
        }
    }
}
