use serde::Deserialize;

use crate::env::{DEF_LOCALES, LOCALES};


#[derive(Deserialize, Clone, Default)]
pub struct TranslateText {
    zh: String,
    en: String,
    jp: String,
}

impl TranslateText {
    pub fn get(&self) -> String {
        unsafe {
            let loc = if !DEF_LOCALES.eq(LOCALES.as_str()) {
                DEF_LOCALES
            } else {
                LOCALES.as_str()
            };

            let text = match loc {
                "en" => &self.en,
                "zh" => &self.zh,
                "ja" => &self.jp,
                _ => &self.en,
            };

            if !text.is_empty() {
                text.to_string()
            } else {
                self.en.clone()
            }
        }
    }
}

pub struct TranslateRegionForm {
    name: TranslateText,
    form: Vec<String>,
}

impl From<TranslateRegionForm> for TranslateText {
    fn from(t: TranslateRegionForm) -> Self {
        let forms = t.form
            .iter()
            .map(translate_region_form);

        TranslateText {
            zh: format!(
                "{} - {}",
                &t.name.zh,
                &forms
                    .clone()
                    .fold(String::new(), |mut acc, t| {
                        acc.push_str(" - ");
                        acc.push_str(&t.zh);
                        acc
                    })
            ),
            en: format!(
                "{} - {}",
                &t.name.en,
                &forms
                    .clone()
                    .fold(String::new(), |mut acc, t| {
                        acc.push_str(" - ");
                        acc.push_str(&t.en);
                        acc
                    })
            ),
            jp: format!(
                "{} - {}",
                &t.name.jp,
                &forms.fold(String::new(), |mut acc, t| {
                    acc.push_str(" - ");
                    acc.push_str(&t.jp);
                    acc
                })
            ),
        }
    }
}

fn translate_region_form<T: AsRef<str>>(form: T) -> TranslateText {
    match form.as_ref() {
        "Alola" => TranslateText {
            zh: "阿羅拉的樣子".to_string(),
            en: "Alola Form".to_string(),
            jp: "アローラのすがた".to_string()
        },
        "Galar" => TranslateText {
            zh: "伽勒爾的樣子".to_string(),
            en: "Galarian form".to_string(),
            jp: "ガラルのすがた".to_string()
        },
        "Hisui" => TranslateText {
            zh: "洗翠的樣子".to_string(),
            en: "Hisuian form".to_string(),
            jp: "ヒスイのすがた".to_string()
        },
        "Paldea" => TranslateText {
            zh: "帕底亞的樣子".to_string(),
            en: "Paldea form".to_string(),
            jp: "パルデアのすがた".to_string()
        },
        _ => TranslateText::default()
    }
}
