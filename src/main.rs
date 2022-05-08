use std::io;
use std::io::{Write};
use rand::Rng;

fn main() {
    println!("Welcome to Rave!");
    let mut character = Character::create_character();
    println!("Created character: {:#?}", character);
}

#[derive(Debug)]
struct Character {
    name: String,
    money: i32,
    stamina: i32,
    health: i32,
    strength: i32,
}

impl Character {
    fn create_character() -> Character {
        let mut name = String::new();
        let mut stamina = String::new();
        let mut strength = String::new();
        let mut attribute_points_to_spend = 10;

        print!("Enter your name:");
        io::stdout().flush().expect("Flush didn't work");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("You have {} attribute points to spend", attribute_points_to_spend);

        let mut parsed_stamina: i32;

        loop {
            print!("Enter health attribute:");
            io::stdout().flush().expect("Flush didn't work");
            io::stdin().read_line(&mut stamina).expect("Failed to read line");
            parsed_stamina = stamina.trim().parse().expect("Health is not a number");

            if parsed_stamina <= attribute_points_to_spend {
                attribute_points_to_spend -= parsed_stamina;
                break;
            } else {
                println!("Entered health must be less than available attributes");
                stamina.clear();
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
            stamina: parsed_stamina,
            strength: parsed_strength,
            health: stamina * 10,
        };
    }
}

struct Enemy {
    stamina: i32,
    health: i32,
    strength: i32,
    gold: i32,
}

impl Enemy {
    fn resolve_attack(&mut self, character: &mut Character) {
        self.health -= character.strength * (rand::thread_rng().gen_range(1..11) / 10);

        if self.health <= 0 {
            return;
        }
    }
}

struct Encounter {}

impl Encounter {
    fn start_encounter() {
        //spawn enemy
        //call resolve attack on enemy until enemy dies
    }
}