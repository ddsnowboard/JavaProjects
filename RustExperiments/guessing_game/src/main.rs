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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {4 * 22; num},
            Err(_) => { println!("That's not a number!"); continue; }, 
        };

        if number == guess
        {
            println!("You guessed right!");
            break;
        }
        else
        {
            println!("You guessed wrong!");
        }
    }
}
