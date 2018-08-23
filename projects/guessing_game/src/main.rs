extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_a_guess() -> u8 {
    println!("Please, input your guess:");

    let mut guess = String::new();

    loop {
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        // wow! shadowing. It needs to understand, if it's nice to use it
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        return guess;
    }
}

fn check_guess(secret: u8, guess: u8) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("Good job!");
            return true;
        },
    }
    return false;
}

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1, 101);
    loop {
        let guess = get_a_guess();
        if check_guess(secret_number, guess) {
            break;
        }
    }
}