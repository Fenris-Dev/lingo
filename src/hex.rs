use crate::Translator;

pub struct Hexadecimal {}
impl Translator for Hexadecimal {
    fn from_english(&self, text: &str) -> String {
        text.bytes()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    fn to_english(&self, text: &str) -> String {
        let bytes:Vec<u8> = text
            .as_bytes()
            .chunks(2)
            .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect();
        String::from_utf8_lossy(&bytes).to_string()
    }

    fn is_language(&self, text: &str) -> bool {
        for c in text.chars() {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
        true
    }
}
