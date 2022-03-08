// We need to take in user input so bring in this library
use std::io;
// Bring ordering (an ennum) a result of two things being compared
use std::cmp::Ordering;
// Bring Rng trait into scope
use rand::Rng;
// Bring colored library to scope
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // This loop let's user to keep guessing without exiting the app
    loop {
        println!("Please input your guess.");

        //Make guess mutable           ,new() is like a method that creates a new string
        let mut guess = String::new();

        io::stdin()
            // Takes mutable reference to string and modify without taking ownership
            .read_line(&mut guess)
            // This function will return the value otherwise return a panic with msg passed in
            .expect("Failed to read line");

        // We do shadowing to guess
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
    }

    
}