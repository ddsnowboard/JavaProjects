extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1, 101);
    println!("Pssst! The number is {}", number);

    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("Input your guess: ");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u32>() {
            Ok(num) if num == number => {
                println!("You guessed right!");
                return;
            },
            Ok(_) => {
                println!("You guessed wrong!");
            },
            Err(_) => {
                println!("That's not a number!");
                continue;
            } 
        };
    }
}
