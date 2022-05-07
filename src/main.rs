use std::io;
use std::io::{Write};

fn main() {
    println!("Welcome to Rave!");
    let mut character = Character::create_character();
    println!("Created character: {:#?}", character);
}

#[derive(Debug)]
struct Character {
    name: String,
    money: i32,
    health: i32,
    strength: i32,
}

impl Character {
    fn create_character() -> Character {
        let mut name = String::new();
        let mut health = String::new();
        let mut strength = String::new();

        print!("Enter your name:");
        io::stdout().flush().expect("Flush didn't work");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        print!("Enter health attribute:");
        io::stdout().flush().expect("Flush didn't work");
        io::stdin().read_line(&mut health).expect("Failed to read line");

        print!("Enter strength attribute:");
        io::stdout().flush().expect("Flush didn't work");
        io::stdin().read_line(&mut strength).expect("Failed to read line");

        return Character {
            name: String::from(name.trim()),
            money: 0,
            health: health.trim().parse::<i32>().expect("Not a number"),
            strength: strength.trim().parse::<i32>().expect("Not a number"),
        };
    }
}