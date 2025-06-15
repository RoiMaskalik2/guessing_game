use rand::{rng, Rng};
use std::cmp::Ordering;
use std::io;

const GENERATED_NUMBER_UPPER_LIMIT: i32 = 100;
const GENERATED_NUMBER_LOWER_LIMIT: i32 = 1;

fn main() {
    print_welcome_message();

    let generated_number: i32 = generate_random_number();

    run_guessing_game_loop(generated_number);
}

/// This function prints the game instructions to the user
fn print_welcome_message() {
    println!(
        "Welcome To Guess The Number!\n You Should enter a number between {}-{}",
        GENERATED_NUMBER_LOWER_LIMIT, GENERATED_NUMBER_UPPER_LIMIT
    );
}

/// This function randomly generates a number between 1-100 for the user to guess
///
/// # Returns
///
/// The generated number for the user to guess
fn generate_random_number() -> i32 {
    return rng().random_range(GENERATED_NUMBER_LOWER_LIMIT..=GENERATED_NUMBER_UPPER_LIMIT);
}

/// This function runs the main loop of the guessing game
/// The loop receives input from the user until they guess the correct generated number
///
/// # Arguments
///
/// * `generated_numebr` - a randomly generated number that the user needs to guess
fn run_guessing_game_loop(generated_number: i32) {
    loop {
        let user_guess = match receive_user_guess() {
            Some(guess) => guess,
            None => continue,
        };

        // Compare the guessed number to the generated number
        match user_guess.cmp(&generated_number) {
            Ordering::Less => println!("The Guessed number is too small"),
            Ordering::Greater => println!("The Guessed number is too big"),
            Ordering::Equal => {
                println!("Congrats! You have guessed the right number!");
                break;
            }
        }
    }
}

/// This function receives a guess from the user and validates that it is an integer between 1-100
///
/// # Returns
///
/// The input that was received from the user if the user guess is valid (An integer between 1-100)
/// 'None' Otherwise
fn receive_user_guess() -> Option<i32> {
    println!("Please enter your guess");

    // Receive input from the user
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line from standard input, Please check your Termimal");

    // Validate the user input
    let guess: Option<i32> = match user_input.trim().parse() {
        Ok(converted_input)
            if (GENERATED_NUMBER_LOWER_LIMIT..=GENERATED_NUMBER_UPPER_LIMIT)
                .contains(&converted_input) =>
        {
            Some(converted_input)
        }
        Ok(_) => {
            println!(
                "Please enter a number between {}-{}",
                GENERATED_NUMBER_LOWER_LIMIT, GENERATED_NUMBER_UPPER_LIMIT
            );
            None
        }
        Err(_) => {
            println!("Invalid Input! Please enter a number");
            None
        }
    };

    return guess;
}
