use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Create a mutable String to store the guess
        let mut guess = String::new();

        // Read the input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Trim the input and print the guess
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("too big"),
        }
    }
}
