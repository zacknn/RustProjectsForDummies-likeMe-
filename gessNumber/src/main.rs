use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");

    // Generate secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    loop {
        println!("Please enter your guess:");

        // Read user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Increment guess counter
        guess_count += 1;

        println!("You guessed: {}", guess);

        // Compare guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed it right!");
                println!("It took you {} guesses!", guess_count);
                break;
            }
        }
    }
}

