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