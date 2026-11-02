use std::io;

fn get_input() -> String { // This function gets input from the user and returns it as a String
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn change_string_to_u8(input: &str) -> Result<u8, std::num::ParseIntError> { // This function converts a string to a u8 number and returns a Result
    input.parse::<u8>()
}

fn check_guess(guess: u8, golden_number: u8) -> bool { // This function checks if the guess is correct
    if guess < golden_number {
        println!("Too low! Try again.");
        false
    } else if guess > golden_number {
        println!("Too high! Try again.");
        false
    } else {
        println!("Congratulations! You guessed the number!");
        true
    }
}

fn main() {
    println!("Welcome to the Guess the Number game!");
    println!("Computer will generate a random number between 1 and 100, and you have to guess it.");

    let golden_number: u8 = rand::random_range(1..=100); // Generate random number between 1 and 100

    loop {
        println!("Enter your guess:");
        let input = get_input();
        let guess: u8 = match change_string_to_u8(&input) {
            Ok(guess) => guess,
            Err(error) => {
                println!("Error: {error}");
                return;
            }
        };

        if check_guess(guess, golden_number) {
            break;
        }
    }
}