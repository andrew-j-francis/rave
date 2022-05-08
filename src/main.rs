use std::io;
use std::io::{Write};
use rand::Rng;

fn main() {
    println!("Welcome to Rave!");
    let mut character = Character::create_character();
    println!("Created character: {:#?}", character);
    Encounter::start_encounter(&mut character);
}


#[derive(Debug)]
struct Character {
    name: String,
    gold: i32,
    stamina: i32,
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
            gold: 0,
            stamina: parsed_stamina,
            strength: parsed_strength,
        };
    }
}


struct Enemy {
    stamina: i32,
    strength: i32,
    gold: i32,
}

impl Enemy {
    fn spawn_enemy() -> Enemy {
        return Enemy {
            stamina: rand::thread_rng().gen_range(4..8),
            strength: rand::thread_rng().gen_range(3..6),
            gold: rand::thread_rng().gen_range(4..16),
        };
    }
}


struct Encounter {}

impl Encounter {
    fn start_encounter(character: &mut Character) {
        let mut enemy = Enemy::spawn_enemy();

        if Encounter::resolve_encounter(character, &mut enemy) {
            println!("Enemy died");
            println!("The enemy dropped {} gold", enemy.gold);

            character.gold += enemy.gold;

            println!("You now have {} gold.", character.gold);
        } else {
            println!("You died");
        }
    }

    fn resolve_encounter(character: &mut Character, enemy: &mut Enemy) -> bool {
        let mut enemy_health: i32 = enemy.stamina * 5;
        let mut character_health: i32 = character.stamina * 5;

        let enemy_attack_power = enemy.strength * 10;
        let character_attack_power = character.strength * 10;

        println!("********** New Encounter **********");
        println!("***** Enemy Health: {}", enemy_health);
        println!("***** Enemy AP: {}", enemy_attack_power);

        loop {
            enemy_health -= character_attack_power;

            println!("You hit for {} damage.", character_attack_power);
            println!("Enemy has {} health left.", enemy_health);

            if enemy_health <= 0 {
                return true;
            }

            character_health -= enemy_attack_power;

            println!("The enemy hits for {} damage.", enemy_attack_power);
            println!("You have {} health left.", character_health);

            if character_health <= 0 {
                return false;
            }
        }
    }
}