use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AsciiJsonItem {
    pub name: String,
    pub forms: Vec<String>,
}

impl AsciiJsonItem {
    fn new(name: String, forms: Vec<String>) -> Self {
        Self { name, forms }
    }
}

pub type AsciiJson = Vec<AsciiJsonItem>;

#[derive(Default)]
pub struct AsciiForms(BTreeMap<String, Vec<String>>);

impl From<AsciiJson> for AsciiForms {
    fn from(value: AsciiJson) -> Self {
        Self(
            value
                .into_iter()
                .map(|item| {
                    AsciiJsonItem::new(
                        item.name,
                        item.forms
                            .into_iter()
                            .chain(vec![String::from("shiny")])
                            .collect()
                    )
                })
                .fold(BTreeMap::new(), |mut map, item| {
                    map.insert(item.name, item.forms);
                    map
                }),
        )
    }
}

impl AsciiForms {
    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.0.get(name)
    }
}

