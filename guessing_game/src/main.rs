use std::{i64, io};
use rand::Rng;

// fn input_parser(guess: String) -> i64{
//     let guessed_value: i64 = guess.trim()
//                                 .parse()
//                                 .expect("Input cannot be parsed to integer");
//     return guessed_value
// }

enum ParseResult {
    Integer(i64),
    Error(&'static str)
}

fn input_parser_with_enum(guess: String) -> ParseResult {
    match guess.trim().parse::<i64>() {
        Ok(x) => ParseResult::Integer(x),
        Err(_) => ParseResult::Error("Unable to parse to integer")
    }
}

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let max_guess = 5;
    let min_guess = 1;

    let mut is_guess_correctly: bool = false;

    while is_guess_correctly == false {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guessed_value = input_parser(guess.clone());
        let secret_number = rand::thread_rng().gen_range(min_guess..=max_guess);

        // if guessed_value == secret_number {
        //     println!("You guessed correctly! The secret number is {secret_number}");
        //     is_guess_correctly = true;
        // } else if guessed_value > max_guess || guessed_value < min_guess {
        //     println!("You might want to guess between {0} and {1}", min_guess, max_guess);
        // } else {
        //     println!("You guessed wrong. The secret number is {secret_number}");
        // }

        // if is_guess_correctly == false {
        //     println!("Try to guess again");
        // }

        match input_parser_with_enum(guess.clone()) {
            ParseResult::Integer(guessed_value) => {
                if guessed_value == secret_number {
                    println!("You guessed correctly! The secret number is {secret_number}");
                    is_guess_correctly = true;
                } else if guessed_value > max_guess || guessed_value < min_guess {
                    println!("You might want to guess between {0} and {1}", min_guess, max_guess);
                } else {
                    println!("You guessed wrong. The secret number is {secret_number}");
                }
            }
            ParseResult::Error(error_message) => {
                println!("Error: {}", error_message);
            }
        }
    }
    
}