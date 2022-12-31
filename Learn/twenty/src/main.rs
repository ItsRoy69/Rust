// Generate a secret number with Guessing Game 

// Using rand dependency in cargo.toml ro generate rnadom numbe as rust doesnt allow to generate random number


extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);    
}
