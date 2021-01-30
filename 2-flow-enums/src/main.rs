// Imports
use std::env;
use std::io::{self};
use rand::prelude::*;

use std::cmp::{Ordering};

// Constants
const MAX_RANGE: u32 = 100;
const MAX_ROUNDS: u8 = 3;

/// Different cases between a user's guess and the actual value
enum GuessEnum {
    /// Guess was too low with difference between actual value and guess
    TooLow(u32),
    /// Guess was high low with difference between actual value and guess
    TooHigh(u32),
    /// Guess was the exact match
    Exact,
}

// Parse a valid integer from the input
fn parse_int(int_str: &str) -> Option<u32> {
    match int_str.parse::<u32>() {
        Ok(val) => Some(val),
        Err(_e) => {
            // println!("Error parsing int_str: {}\nGot error: {}", int_str, e);
            None
        }
    }
}

// Check for valid response
fn judge_guess(guess: u32, actual: u32) -> GuessEnum {
    match guess.cmp(&actual) {
        Ordering::Greater => GuessEnum::TooHigh(guess - actual),
        Ordering::Less => GuessEnum::TooLow(actual - guess),
        Ordering::Equal => GuessEnum::Exact,
    }
}

// Wrap the game loop 
fn game_loop(actual: u32, baby_mode: bool) -> Result<u32, String> {
    // Counter
    let mut num_guesses = 1;

    // Loops infinitely until broken
    loop {
        println!("Please enter a number from 1 to {}:", MAX_RANGE);

        // Get user input
        let mut guess_line = String::new();
        let result = io::stdin().read_line(&mut guess_line);
        let guess_str = guess_line.trim();

        // Catch error in obtaining user input
        if result.is_err() {
            return Err(format!("Could not read STDIN. Error: {}", result.unwrap_err()));
        }

        // If a valid input is parsed, assign it to guess, otherwise, ask user to try again.
        let guess = if let Some(num) = parse_int(guess_str) {
            num
        } else {
            println!("Please enter a valid number!");
            continue; // Don't count a guess if it's not valid
        };

        // Match the guess to determine if we need to loop again or not
        match judge_guess(guess, actual) {
            GuessEnum::TooHigh(diff) => {
                if baby_mode {
                    println!("Oh no thats too high. You were off by {}", diff);
                } else {
                    println!("Oh no thats too high.");
                }
            },
            GuessEnum::TooLow(diff) => {
                match baby_mode {
                    true => println!("Oh no thats too low. You were off by {}", diff),
                    false => println!("Oh no thats too low."),
                }
            },
            GuessEnum::Exact => return Ok(num_guesses),
        }

        num_guesses += 1; // increment counter
    }
}

fn main() {
    // Parse command args
    let baby_mode = env::args().any(|i| i.trim() == "--baby"); // The program will alert the player how far off they are

    // Play a new game for each round
    for i in 0..MAX_ROUNDS {
        // Preamble text
        println!("Starting round {} of {}", i + 1, MAX_ROUNDS);
        println!("I'm thinking of a number from 0 to {}", MAX_RANGE);

        // Generate the random numer
        let mut rng = rand::thread_rng();
        let actual = rng.gen_range(0..MAX_RANGE);

        // Catches exceptions in the main game loop
        match game_loop(actual, baby_mode) {
            Ok(num_guesses) => println!("Wow you got it in only {} guesses!\n", num_guesses),
            Err(err) => println!("Hmmm found an error: {}\n", err),
        };
    }

    // Post-game
    println!("Thank you for playing!");
}
