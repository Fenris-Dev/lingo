use lingo::{*, binery::Binary, morse::MorseCode, hex::Hexadecimal, braille::Braille};

fn main() 
{


    let module = Binary {};
    let text = "Hello, world!";
    println!("bny: {} | {:?}", module.is_language(&text),&text);
    let text = module.from_english(&text);
    println!("bny: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("bny: {} | {:?}", module.is_language(&text),&text);

    println!("");
    
    let module = MorseCode::new();
    let text = "Hello, world!";
    println!("mor: {} | {:?}", module.is_language(&text),&text);
    let text = module.from_english(&text);
    println!("mor: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("mor: {} | {:?}", module.is_language(&text),&text);

    println!("");

    let module = Hexadecimal {};
    let text = "hello, world!";
    println!("hex: {} | {:?}", module.is_language(&text),&text);
    let text = module.from_english(&text);
    println!("hex: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("hex: {} | {:?}", module.is_language(&text),&text);

    println!("");

    let module = Braille::new();
    let text = "hello, world!";
    println!("brl: {} | {:?}", module.is_language(&text),&text);
    let text = module.from_english(&text);
    println!("brl: {} | {:?}", module.is_language(&text),&text);
    let text = module.to_english(&text);
    println!("brl: {} | {:?}", module.is_language(&text),&text);

}


//Braille
//Semaphore
