use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {secret_number}"); // Debugging purpose, should be commented out or removed in a real game

    println!("Please input your guess.");

    // Create a mutable String to store the guess
    let mut guess = String::new();

    // Read the input from the user
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Trim the input and print the guess
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Equal => println!("you win"),
        Ordering::Greater => println!("too big");  
    }
}
