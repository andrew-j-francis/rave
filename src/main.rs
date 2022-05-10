mod character;

use std::io;
use std::io::{Write};
use rand::Rng;
use cursive::views::{CircularFocus, Dialog, DummyView, EditView, LinearLayout, SliderView, TextView};
use cursive::{Cursive, View, With};
use cursive::align::HAlign;
use cursive::reexports::log::warn;
use cursive::view::{Nameable, Resizable, Scrollable};

fn main() {
    let mut siv = cursive::default();
    siv.load_toml(include_str!("style.toml")).unwrap();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Character Name"))
                .child(EditView::new())
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Strength"))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("0").with_name("strength_amount"))
                        .child(DummyView.fixed_width(1))
                        .child(
                            SliderView::horizontal(11)
                                .on_change(|s, v| {
                                    s.call_on_name("strength_amount", |view: &mut TextView| {
                                        view.set_content(v.to_string());
                                    });
                                })
                        )
                )
                .child(TextView::new("Stamina"))
                .child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::new("0").with_name("stamina_amount")
                        )
                        .child(DummyView.fixed_width(1))
                        .child(
                            SliderView::horizontal(11)
                                .on_change(|s, v| {
                                    s.call_on_name("stamina_amount", |view: &mut TextView| {
                                        view.set_content(v.to_string());
                                    });
                                })
                        )
                )
                .fixed_width(30),
        )
            .title("Create Character")
            .button("Create", |s| s.quit())
            .h_align(HAlign::Left),
    );

    siv.run();
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