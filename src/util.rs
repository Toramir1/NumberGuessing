use std::io;
use std::io::stdout;
use crossterm::execute;
use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType};

pub fn clear_console() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn print_colored_message(msg: &str, color: Color) {
    println!("{}", msg.with(color));
}

pub fn print_error(msg: &str) {
    print_colored_message(msg, Color::Red);
}

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input
}