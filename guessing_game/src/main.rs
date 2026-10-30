use std::io;

use std::cmp::Ordering;

use rand::prelude::*;

fn main() {
    let mut guess = String::new();

    let secret_num = rand::rng().random_range(1..=100);

    println!("Guess the number!");

    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win!"),
    }
}
