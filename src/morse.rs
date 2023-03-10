use crate::Translator;
use std::collections::HashMap;

pub struct MorseCode {
    map: HashMap<char, &'static str>,
} impl MorseCode {
    pub fn new() -> Self {
        let map: HashMap<char, &'static str> = [
            ('a', ".-"),
            ('b', "-..."),
            ('c', "-.-."),
            ('d', "-.."),
            ('e', "."),
            ('f', "..-."),
            ('g', "--."),
            ('h', "...."),
            ('i', ".."),
            ('j', ".---"),
            ('k', "-.-"),
            ('l', ".-.."),
            ('m', "--"),
            ('n', "-."),
            ('o', "---"),
            ('p', ".--."),
            ('q', "--.-"),
            ('r', ".-."),
            ('s', "..."),
            ('t', "-"),
            ('u', "..-"),
            ('v', "...-"),
            ('w', ".--"),
            ('x', "-..-"),
            ('y', "-.--"),
            ('z', "--.."),
            ('0', "-----"),
            ('1', ".----"),
            ('2', "..---"),
            ('3', "...--"),
            ('4', "....-"),
            ('5', "....."),
            ('6', "-...."),
            ('7', "--..."),
            ('8', "---.."),
            ('9', "----."),
            ('.', ".-.-.-"),
            (',', "--..--"),
            ('?', "..--.."),
            ('\'', ".----."),
            ('!', "-.-.--"),
            ('/', "-..-."),
            ('(', "-.--."),
            (')', "-.--.-"),
            ('&', ".-..."),
            (':', "---..."),
            (';', "-.-.-."),
            ('=', "-...-"),
            ('+', ".-.-."),
            ('-', "-....-"),
            ('_', "..--.-"),
            ('"', ".-..-."),
            ('$', "...-..-"),
            ('@', ".--.-."),
            (' ', "/"),
        ].iter().cloned().collect();

        Self { map }
    }
    
    pub fn get_by_key(&self, value: &str) -> Option<String>{
        let v:Option<String> = self.map.iter()
            .filter(|(_, &v)| v == value)
            .map(|(k, _)| Some(format!("{}", k.clone())))
            .collect();
        v
    }
}

impl Translator for MorseCode {
    fn from_english(&self, text: &str) -> String {
        text.to_lowercase().chars()
            .map(|c| self.map.get(&c).cloned())
            .filter(|o| o.is_some())
            .map(|o| format!("{} ",o.unwrap()))
            .collect::<Vec<String>>()
            .join("")
    }

    fn to_english(&self, text: &str) -> String {
        text.split(' ')
            .map(|morse| self.get_by_key(morse))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect::<Vec<String>>()
            .join("")
    }

    fn is_language(&self, text: &str) -> bool {
        for c in text.chars(){
            if (c != '.') && (c != '-') && (c != ' ') && (c != '/') {
                return false;
            }
        }
        true
    }
}
