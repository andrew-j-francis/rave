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
        let mut attribute_points_to_spend = 10;

        print!("Enter your name:");
        io::stdout().flush().expect("Flush didn't work");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("You have {} attribute points to spend", attribute_points_to_spend);

        let mut parsed_health: i32;

        loop {
            print!("Enter health attribute:");
            io::stdout().flush().expect("Flush didn't work");
            io::stdin().read_line(&mut health).expect("Failed to read line");
            parsed_health = health.trim().parse().expect("Health is not a number");

            if parsed_health <= attribute_points_to_spend {
                attribute_points_to_spend -= parsed_health;
                break;
            } else {
                println!("Entered health must be less than available attributes");
                health.clear();
            }
        }

        println!("You have {} attribute points to spend", attribute_points_to_spend);

        let mut parsed_strength: i32;
        loop {
            print!("Enter strength attribute:");
            io::stdout().flush().expect("Flush didn't work");
            io::stdin().read_line(&mut strength).expect("Failed to read line");
            parsed_strength = strength.trim().parse().expect("Strength is not a number");

            if parsed_strength <= attribute_points_to_spend {
                break;
            } else {
                println!("Entered strength must be less than available attributes");
                strength.clear();
            }
        }

        return Character {
            name: String::from(name.trim()),
            money: 0,
            health: parsed_health,
            strength: parsed_strength,
        };
    }
}