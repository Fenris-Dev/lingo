use crate::Translator;

pub struct Binary {}
impl Translator for Binary {
    fn from_english(&self, text: &str) -> String {
        text.chars()
            .map(|c| format!("{:08b}", c as u8))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn to_english(&self, text: &str) -> String {
        text.split(' ')
            .map(|byte| u8::from_str_radix(byte, 2).unwrap())
            .filter(|&byte| byte !=0)
            .map(|byte| byte as char)
            .collect::<String>()
    }

    fn is_language(&self, text: &str) -> bool {
        for c in text.chars(){
            if (c != '0') && (c != '1') && (c != ' ') {
                return false;
            }
        }
        true
    }
}
