use lazy_static::lazy_static;
use sys_locale::get_locale;

pub static LIST_H_MARGIN: u16 = 2;
pub static mut DEF_LOCALES: &str = "en";

lazy_static! {
    pub static ref LOCALES: String = get_locale()
        .map(|s| { s.split('-').next().unwrap_or("en").to_string() })
        .unwrap_or_else(|| "en".to_string());
}
