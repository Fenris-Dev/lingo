pub mod binery;
pub mod morse;

pub trait Translator {
    fn from_english(&self, text: &str) -> String;
    fn to_english(&self, text: &str) -> String;
    fn is_language(&self, text: &str) -> bool;
}

