mod util;

use crate::util::{
    clear_console, loss, print_colored_message, print_error, read_user_input, unicorn, win_screen,
};
use crossterm::style::Color;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::RangeInclusive;
use std::process::exit;

const MODE_ONE_SHOT: u8 = 1;
const MODE_REPEAT: u8 = 2;

fn main() {
    clear_console();
    loop {
        println!("Please choose the mode you want to Play \n");

        println!("1. One Shot");
        println!("2. Multiple Guesses");
        println!("3. Info");
        println!("4. Credits");
        println!("5. Exit");

        let selection = read_user_input();

        let parsed_selection: u8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print_error("Invalid input, please enter a number.");
                continue;
            }
        };

        match parsed_selection {
            1 => difficulty_selection(parsed_selection),
            2 => difficulty_selection(parsed_selection),
            3 => info(),
            4 => credits(),
            5 => exit(0),
            69 => secret(),
            _ => {
                print_error("Invalid selection, going back to menu");
                continue;
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

        let guess = read_user_input();

        if guess.trim() == "exit" {
            break;
        }

        println!("You guessed: {}", guess);

        let guess_as_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess_as_number > secret_number {
            print_error("Please enter a number smaller than range");
            continue;
        }

        if guess_as_number == secret_number {
            win_screen();
            break;
        } else {
            clear_console();
            loss();
            println!("The secret number was {}.", secret_number);
            println!("Generated new Random number;");
            break;
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

        let guess = read_user_input();

        if guess.trim() == "exit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if guess > secret_number {
            print_error("Please enter a number smaller than range");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print_colored_message("Too small!", Color::Yellow),
            Ordering::Greater => print_colored_message("Too big!", Color::Yellow),
            Ordering::Equal => {
                win_screen();
                println!("Press Enter to return to the main menu...");
                let _ = io::stdin().read_line(&mut String::new());
                main()
            }
        }
    }
}

fn info() {
    clear_console();
    print_colored_message(
        "The One Shot mode lets you make only a single guess and generates a new random number every time you fail.\n",
        Color::Yellow,
    );

    print_colored_message(
        "The Multiple Guesses mode lets you make guesses until you get the correct number, \
        with hints if your guess was higher or lower than the secret number.\n",
        Color::Yellow,
    );

    await_user_input();
}

fn difficulty_selection(game_mode: u8) {
    println!("Please select the difficulty you want to Play");
    println!("Difficulty affects the range of numbers \n");

    println!("Use the following:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!("4. Impossible");
    println!("5. Custom");

    let selection = read_user_input();

    let mut is_custom: bool = false;

    let range = match selection.trim() {
        "1" => 1..=10,
        "2" => 1..=100,
        "3" => 1..=1000,
        "4" => 1..=1_000_000,
        "5" => {
            println!("What should the highest number of the range be?");

            is_custom = true;

            let high_end = read_user_input();
            let end = high_end.trim().parse().expect("Please type a number!");
            1..=end
        }
        _ => {
            print_error("Invalid selection, defaulting to Medium.");
            1..=100
        }
    };

    match game_mode {
        MODE_ONE_SHOT => one_shot(range),
        MODE_REPEAT => {
            if !is_custom {
                let start = *range.start();
                let end = *range.end() * 10;
                let modified_range = start..=end;
                repeat_guesses(modified_range);
            } else if is_custom {
                repeat_guesses(range);
            }
        }
        _ => {
            print_error("Invalid selection");
        }
    }
}

fn credits() {
    print_colored_message(
        "\nDevelopment: Toramir",
        Color::Rgb {
            r: 195,
            g: 0,
            b: 255,
        },
    );
    print_colored_message("All the other 90% of the Code: ChatGPT", Color::DarkCyan);

    await_user_input();
}

fn secret() {
    unicorn()
}

fn await_user_input() {
    println!("Press Enter to return to the main menu...");
    let _ = io::stdin().read_line(&mut String::new());
    main()
}
