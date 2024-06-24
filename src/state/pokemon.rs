use std::{path::PathBuf, rc::Rc};

use ratatui::widgets::{ListState, ScrollbarState};

use crate::pokemon::{ascii_form::AsciiForms, AbilityMap, PokemonBundle, PokemonEntity};

#[derive(Copy, Clone)]
pub enum AsciiType {
    Normal,
    Shiny,
}

#[derive(Default)]
pub struct PokemonListState {
    pub list_scrollbar_state: ScrollbarState,
    pub list_state: ListState,
    pub filtered_list: Vec<Rc<PokemonEntity>>,
    pub filter_query: String,
    pub desc_scrollbar_state: ScrollableParagraphState,
    pub bundle: Rc<PokemonBundle>,
    pub profile_page: u8,
    pub ascii_root: PathBuf,
    pub ascii_form_map: AsciiForms,
    pub ascii_form_index: usize,
}

impl PokemonListState {
    pub fn new(bundle: Rc<PokemonBundle>, ascii_form_map: AsciiForms) -> Self {
        let pokemon_len = bundle.pokemon.len();
        let list_scrollbar_state = ScrollbarState::default().content_length(pokemon_len);

        let mut list_state = ListState::default();
        list_state.select_first();

        let filtered_list = Vec::with_capacity(pokemon_len);

        Self {
            bundle,
            list_state,
            list_scrollbar_state,
            filtered_list,
            ascii_form_map,
            ..Default::default()
        }
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.ascii_root = path;
        self
    }

    pub fn get_assets_path(&self, t: AsciiType) -> PathBuf {
        self.ascii_root.join(match t {
            AsciiType::Normal => "regular",
            AsciiType::Shiny => "shiny",
        })
    }

    pub fn len(&self) -> usize {
        self.bundle.pokemon.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bundle.pokemon.is_empty()
    }

    pub fn ability_map(&self) -> Rc<AbilityMap> {
        self.bundle.ability.clone()
    }

    pub fn scroll_to_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn scroll_to_end(&mut self) {
        self.list_state.select_last();
    }

    pub fn next(&mut self) {
        self.list_state.select_next();
    }

    pub fn previous(&mut self) {
        self.list_state.select_previous();
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
        self.filter_query.clone_from(&filter);

        if !filter.is_empty() {
            self.filtered_list.clear();
            self.filtered_list.extend(
                self.bundle
                    .pokemon
                    .iter()
                    .filter(|item| {
                        item.name_with_no()
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                    })
                    .cloned(),
            );
        };

        self.select(0);
    }

    pub fn select(&mut self, index: usize) {
        self.profile_page = 0;
        self.desc_scrollbar_state.reset();
        self.list_state.select(Some(index));
        self.list_scrollbar_state = self.list_scrollbar_state.position(index);
        self.reset_ascii_form_index();
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

    pub fn profile(&self) -> Option<Rc<PokemonEntity>> {
        let index = self.list_state.selected()?;
        if self.filter_query.is_empty() {
            self.bundle.pokemon.get(index).cloned()
        } else {
            self.filtered_list.get(index).cloned()
        }
    }

    pub fn profile_with_region_form(&self) -> Option<Rc<PokemonEntity>> {
        let profile = self.profile()?;
        if self.profile_page > 0 {
            profile
                .region_form()?
                .get((self.profile_page as usize).saturating_sub(1))
                .map(|x| Rc::new(x.clone()))
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
        self.reset_ascii_form_index();
        let len = self.region_form_len();
        if len > 0 && self.profile_page < len - 1 {
            self.profile_page = self.profile_page.saturating_add(1);
        }
    }

    pub fn previous_profile_page(&mut self) {
        self.reset_ascii_form_index();
        let len = self.region_form_len();
        if len > 0 && self.profile_page > 0 {
            self.profile_page = self.profile_page.saturating_sub(1);
        }
    }

    pub fn list_items(&self) -> &Vec<Rc<PokemonEntity>> {
        if self.filter_query.is_empty() {
            &self.bundle.pokemon
        } else {
            &self.filtered_list
        }
    }

    pub fn increase_ascii_form_index(&mut self) {
        self.ascii_form_index = if self.ascii_form_index == usize::MAX {
            0
        } else {
            self.ascii_form_index + 1
        }
    }

    pub fn reset_ascii_form_index(&mut self) {
        self.ascii_form_index = 0
    }
}

#[derive(Default)]
pub struct ScrollableParagraphState {
    pub scrollbar_state: ScrollbarState,
    pub position: usize,
    pub height: usize,
}

impl ScrollableParagraphState {
    pub fn set_height(&mut self, height: usize) {
        self.scrollbar_state = self.scrollbar_state.content_length(height);
        self.height = height;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.update_scrollbar();
    }

    pub fn scroll_down(&mut self) {
        if self.position < self.height.saturating_sub(2) {
            self.position = self.position.saturating_add(1);
        }

        self.update_scrollbar();
    }

    pub fn scroll_up(&mut self) {
        self.position = self.position.saturating_sub(1);
        self.update_scrollbar();
    }

    fn update_scrollbar(&mut self) {
        self.scrollbar_state = self.scrollbar_state.position(self.position)
    }
}
