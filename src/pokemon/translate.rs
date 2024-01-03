use serde::Deserialize;

use crate::env::{DEF_LOCALES, LOCALES};

#[derive(Deserialize, Clone, Default)]
pub struct TranslateText {
    pub zh: String,
    pub en: String,
    pub jp: String,
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

impl TranslateRegionForm {
    pub fn new(name: TranslateText, form: Vec<String>) -> Self {
        Self { name, form }
    }
}

impl From<TranslateRegionForm> for TranslateText {
    fn from(t: TranslateRegionForm) -> Self {
        let forms = t.form.iter().map(translate_region_form);

        TranslateText {
            zh: format!(
                "{}{}",
                &t.name.zh,
                &forms.clone().fold(String::new(), |mut acc, t| {
                    acc.push_str(" - ");
                    acc.push_str(&t.zh);
                    acc
                })
            ),
            en: format!(
                "{}{}",
                &t.name.en,
                &forms.clone().fold(String::new(), |mut acc, t| {
                    acc.push_str(" - ");
                    acc.push_str(&t.en);
                    acc
                })
            ),
            jp: format!(
                "{}{}",
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
            jp: "アローラのすがた".to_string(),
        },
        "Galar" => TranslateText {
            zh: "伽勒爾的樣子".to_string(),
            en: "Galarian form".to_string(),
            jp: "ガラルのすがた".to_string(),
        },
        "Hisui" => TranslateText {
            zh: "洗翠的樣子".to_string(),
            en: "Hisuian form".to_string(),
            jp: "ヒスイのすがた".to_string(),
        },
        "Paldea" => TranslateText {
            zh: "帕底亞的樣子".to_string(),
            en: "Paldea form".to_string(),
            jp: "パルデアのすがた".to_string(),
        },
        "Combat Breed" => TranslateText {
            zh: "鬥戰種".to_string(),
            en: "Combat Breed".to_string(),
            jp: "コンバットしゅ".to_string(),
        },
        "Blaze Breed" => TranslateText {
            zh: "火熾種".to_string(),
            en: "Blaze Breed".to_string(),
            jp: "ブレイズしゅ".to_string(),
        },
        "Aqua Breed" => TranslateText {
            zh: "水瀾種".to_string(),
            en: "Aqua Breed".to_string(),
            jp: "ウォーターしゅ".to_string(),
        },
        "Zen Mode" => TranslateText {
            zh: "達摩模式".to_string(),
            en: "Zen Mode".to_string(),
            jp: "ダルマモード".to_string(),
        },
        "Red-Striped" => TranslateText {
            zh: "紅條紋".to_string(),
            en: "Red-Striped".to_string(),
            jp: "あかすじのすがた".to_string(),
        },
        "White-Striped" => TranslateText {
            zh: "白條紋".to_string(),
            en: "White-Striped".to_string(),
            jp: "しろすじのすがた".to_string(),
        },
        "Blue-Striped" => TranslateText {
            zh: "藍條紋".to_string(),
            en: "Blue-Striped".to_string(),
            jp: "あおすじのすがた".to_string(),
        },
        _ => TranslateText::default(),
    }
}
