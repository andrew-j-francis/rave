mod character;

use std::io;
use std::io::{Write};
use console::{Alignment, pad_str, style};
use rand::Rng;

fn main() {
    println!("Welcome to {}!", style("Rave").red());

    let mut character = character::create_character();
    println!("Created character: {:#?}", character);
    Encounter::start_encounter(&mut character);
}

struct Encounter {}

impl Encounter {
    fn start_encounter(character: &mut character::Character) {
        let mut enemy = character::spawn_enemy();

        if Encounter::resolve_encounter(character, &mut enemy) {
            println!("Enemy died");
            println!("The enemy dropped {} gold", enemy.gold);

            character.gold += enemy.gold;

            println!("You now have {} gold.", character.gold);
        } else {
            println!("You died");
        }
    }

    fn resolve_encounter(character: &mut character::Character, enemy: &mut character::Character) -> bool {
        let mut enemy_health: i32 = enemy.stamina * 5;
        let mut character_health: i32 = character.stamina * 5;

        let enemy_attack_power = enemy.strength * 10;
        let character_attack_power = character.strength * 10;

        println!("********** New Encounter **********");
        print!("{}", style(pad_str("Health", 15, Alignment::Left, None)).black().on_white());
        println!("{}", style(pad_str("Attack Power", 15, Alignment::Left, None)).black().on_white());

        print!("{}", pad_str(&enemy_health.to_string(), 15, Alignment::Left, None));
        println!("{}", pad_str(&enemy_attack_power.to_string(), 15, Alignment::Left, None));

        println!("********** Combat Log **********");
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