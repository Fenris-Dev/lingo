use std::collections::HashMap;
use std::process;
use lingo::{*, binery::Binary, morse::MorseCode, hex::Hexadecimal, braille::Braille};
use clap::Parser;
use clap::{arg, Command, Arg};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() 
{
    //let args = Args::parse();

    let matches = Command::new("lingo")
        .version("0.1.0")
        .author("Jquirky, @jquirky13")
        .about("translates stuff") 
        .arg(Arg::new("input"))
        .arg(arg!(--to <VALUE>).required(true))
        .get_matches();
   
    let input = matches.get_one::<String>("input").expect("required");
    let to = matches.get_one::<String>("to").expect("required");

    println!("To: {}", to); 
    println!(" ");

    let mut map: HashMap<String, Box<dyn Translator>> = HashMap::new(); 
    map.insert("binary".to_string(), Box::new(Binary {}));
    map.insert("morse".to_string(), Box::new(MorseCode::new()));
    map.insert("hex".to_string(), Box::new(Hexadecimal {}));
    map.insert("braille".to_string(), Box::new(Braille::new()));

    let lang = map.get(to);
    let lang = match lang {
        Some(v) => v,
        None => {
            eprintln!("Unknown language module: {}", to);
            println!("valid modules include:");

            for (k, v) in map.iter(){
                println!(" {}", k);
            }

            process::exit(1);
        },
    };

    let text = lang.from_english(input);
    let check = lang.to_english(&text);

    println!("Result: {}", text);

    println!(" ");
    println!("Input: {}", input);
    println!("OutIn: {}", check);
    println!("match: {}  |  len: {}", input.eq(&check),input.len() == check.len());
    /*
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
*/
}


//Braille
//Semaphore
