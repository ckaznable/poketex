use sys_locale::get_locale;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOCALES: String = get_locale().unwrap_or_else(|| String::from("en-US"));
}
