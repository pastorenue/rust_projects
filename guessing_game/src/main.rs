#![allow(dead_code)]
use std::io;
use std::cmp::Ordering;
use rand::Rng;

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String
}

/// .
fn something_else() -> String {
    let ferris = SeaCreature{
        species: Species::Clam,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let result: String = match ferris.species {
        Species::Clam => String::from("clam"),
        Species::Crab => String::from("crab"),
        Species::Fish => String::from("fish"),
        Species::Octopus => String::from("octopus"),
    };
    return result.to_owned();
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The sea creature is {}", something_else())
}