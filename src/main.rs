use lingo::{*, binery::Binary, morse::MorseCode};

fn main() 
{
    let module = Binary {};
    let text = "Hello, world!";
    println!("is?: {} | {:?}", module.is_language(&text),&text);
    let text = module.from_english(&text);
    println!("is?: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("is?: {} | {:?}", module.is_language(&text),&text);

    let module = MorseCode::new();
    let text = "Hello, world!";
    let text = module.from_english(&text);
    println!("is?: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("is?: {} | {:?}", module.is_language(&text),&text);
}


//Braille
//Semaphore
//Hexadeciamal
