mod util;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::RangeInclusive;
use std::process::exit;
use crate::util::clear_console;

const MODE_ONE_SHOT: u8 = 1;
const MODE_REPEAT: u8 = 2;

fn main() {
    loop {
        println!("Please choose the mode you want to Play \n");

        println!("Use the following:");
        println!("1. One Shot");
        println!("2. Multiple Guess");
        println!("3. Info");
        println!("4. Exit");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let parsed_selection: u8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                return;
            }
        };

        match parsed_selection {
            1 => difficulty_selection(parsed_selection),
            2 => difficulty_selection(parsed_selection),
            3 => info(),
            4 => exit(0),
            _ => {
                println!("Invalid selection, going back to menu");
            }
        }
    }
}

fn one_shot(range: RangeInclusive<u32>) {
    loop {
        let secret_number: u32 = rand::rng().random_range(range.clone());
        println!(
            "Please input your guess from 1 to {}.",
            range.end().to_formatted_string(&Locale::fr)
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess_as_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess_as_number == secret_number {
            println!("Congratulations! You guessed the secret number.");
            break;
        } else {
            clear_console();
            println!("You guessed {}.", guess_as_number);
            println!("Your guess and the secret number do not match.");
            println!("The secret number was {}", secret_number);
            println!("Generating new Random number;");
        }
    }
}

fn repeat_guesses(range: RangeInclusive<u32>) {
    clear_console();
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(range.clone());

    loop {
        println!(
            "Please input your guess from 1 to {}.",
            range.end().to_formatted_string(&Locale::fr)
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                main()
            }
        }
    }
}

fn info() {
    clear_console();
    println!("The One Shot mode lets you make only a single guess and generates a new random number everytime you fail \n");

    println!(
        "The Multiple Guesses mode lets you make guesses until you get the correct number \
    with hints if your guess was higher or lower than the secret number"
    );

    let _ = io::stdin().read_line(&mut String::new());
}

fn difficulty_selection(gamemode: u8) {
    println!("Please select the difficulty you want to Play");
    println!("Difficulty affects the range of numbers \n");

    println!("Use the following:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!("4. Impossible");
    println!("5. Custom");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let range = match selection.trim() {
        "1" => 1..=10,
        "2" => 1..=100,
        "3" => 1..=1000,
        "4" => 1..=1_000_000,
        "5" => {
            let mut high_end = String::new();
            println!("What should the highest number of the range be?");
            io::stdin()
                .read_line(&mut high_end)
                .expect("Failed to read line");
            let end = high_end.trim().parse().expect("Please type a number!");
            1..=end
        }
        _ => {
            println!("Invalid selection, defaulting to Easy.");
            1..=10
        }
    };

    match gamemode {
        MODE_ONE_SHOT => one_shot(range),
        MODE_REPEAT => {
            let modified_range = *range.start()..=(*range.end() * 10);
            repeat_guesses(modified_range);
        },
        _ => {
            println!("Invalid selection");
        }
    }
}
