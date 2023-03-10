use crate::Translator;
use std::{collections::HashMap, fmt::format};
use unicode_segmentation::UnicodeSegmentation;

pub struct Braille {
    map: HashMap<String, String>,
    map_back: HashMap<String, String>,
} impl Braille {
    pub fn new() -> Self {
        let a = vec![
            " ","a","b","c","d","e","f","g","h","i","j",
            "k","l","m","n","o","p","q","r","s","t",
            "u","v","x","y","z","w",
            "1","2","3","4","5","6","7","8","9","0",
            ".",",","'","\n"
        ];
        let b = vec![
            " ","⠁","⠃","⠉","⠙","⠑","⠋","⠛","⠓","⠊","⠚",
            "⠅","⠇","⠍","⠝","⠕","⠏","⠟","⠗","⠎","⠞",
            "⠥","⠧","⠭","⠽","⠵","⠺",
            "⠼⠁","⠼⠃","⠼⠉","⠼⠙","⠼⠑","⠼⠋","⠼⠛","⠼⠓","⠼⠊","⠼",
            "⠲","⠂","⠄","\n" 
        ];

        assert_eq!(a.len(), b.len(), "should be the same size");


        let mut a_b = HashMap::new();
        for (i, v) in a.iter().enumerate() {
            a_b.insert(v.clone().to_string(), b.get(i).unwrap().clone().to_string());
        }
        let mut b_a = HashMap::new();
        for (i, v) in b.iter().enumerate() {
            b_a.insert(v.clone().to_string(), a.get(i).unwrap().clone().to_string());
        }


        Self { 
            map: a_b,
            map_back: b_a
        }
    }
}

impl Translator for Braille {
    fn from_english(&self, text: &str) -> String {
        text.to_lowercase().graphemes(true)
            .map(|c| self.map.get(c).cloned())
            .filter(|o| o.is_some())
            .map(|o| format!("{}",o.unwrap()))
            .collect::<Vec<String>>()
            .join("")
    }

    fn to_english(&self, text: &str) -> String {
    
        let mut result = String::new();
        for graph in text.graphemes(true) {

            //println!("{}", graph);
            if let Some(x) = self.map_back.get(graph){
                result.push_str(x);
            }
        }

        result
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poem_translate() {
        let poem = r"
A whisper in the wind,
A rustle in the leaves,
The secrets of the earth,
Are what my heart believes.

The stars that shine above,
The moon that guides my way,
In nature's gentle embrace,
I find my peace each day.

For in this world of wonder,
So full of light and love,
I know that I am home,
Beneath the skies above.";

        let t = Braille::new();
        let result = t.from_english(poem);
        let result = t.to_english(&result);
        assert_eq!(poem.to_lowercase(), result);


    }

}
