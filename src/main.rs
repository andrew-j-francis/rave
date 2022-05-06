use std::io;
use std::io::{Write};

fn main() {
    println!("Welcome to Rave!");
    let character: (String, String) = create_character();

    println!("{}{}", character.0, character.1);
}

fn create_character() -> (String, String) {
    let mut name = String::new();
    let mut class = String::new();

    print!("Enter your name:");
    io::stdout().flush().expect("Flush didn't work");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("=== Select a Class ===");
    println!("1. Warrior");
    println!("2. Cleric");
    println!("3. Rogue");
    io::stdin().read_line(&mut class).expect("Failed to read line");

    return (name, class);
}
