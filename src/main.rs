extern crate rand;

use rand::prelude;

use std::io;
fn main() {
    println!("Welcome to the guessing game!");
    println!("Input Guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);

    println!("You guessed: {}", guess);
}
