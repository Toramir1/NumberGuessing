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

pub fn win_screen() {
    print_colored_message("

__   __                     _
\\ \\ / /__  _   _  __      _(_)_ __
 \\ V / _ \\| | | | \\ \\ /\\ / / | '_ \\
  | | (_) | |_| |  \\ V  V /| | | | |
  |_|\\___/ \\__,_|   \\_/\\_/ |_|_| |_|

", Color::Green)
}

pub fn loss() {
    print_colored_message("__   __            _
\\ \\ / /__  _   _  | |    ___   ___  ___  ___
 \\ V / _ \\| | | | | |   / _ \\ / _ \\/ __|/ _ \\
  | | (_) | |_| | | |__| (_) | (_) \\__ \\  __/
  |_|\\___/ \\__,_| |_____|\\___/ \\___/|___/\\___|
", Color::Red)
}

pub fn unicorn() {
    print_colored_message("\
                                                        /
                                                  .7
                                       \\       , //
                                       |\\.--._/|//
                                      /\\ ) ) ).'/
                                     /(  \\  // /
                                    /(   J`((_/ \\
                                   / ) | _\\     /
                                  /|)  \\  eJ    L
                                 |  \\ L \\   L   L
                                /  \\  J  `. J   L
                                |  )   L   \\/   \\
                               /  \\    J   (\\   /
             _....___         |  \\      \\   \\```
      ,.._.-'        '''--...-||\\     -. \\   \\
    .'.=.'                    `         `.\\ [ Y
   /   /                                  \\]  J
  Y / Y                                    Y   L
  | | |          \\                         |   L
  | | |           Y                        A  J
  |   I           |                       /I\\ /
  |    \\          I             \\        ( |]/|
  J     \\         /._           /        -tI/ |
   L     )       /   /'-------'J           `'-:.
   J   .'      ,'  ,' ,     \\   `'-.__          \\
    \\ T      ,'  ,'   )\\    /|        ';'---7   /
     \\|    ,'L  Y...-' / _.' /         \\   /   /
      J   Y  |  J    .'-'   /         ,--.(   /
       L  |  J   L -'     .'         /  |    /\\
       |  J.  L  J     .-;.-/       |    \\ .' /
       J   L`-J   L____,.-'`        |  _.-'   |
        L  J   L  J                  ``  J    |
        J   L  |   L                     J    |
         L  J  L    \\                    L    \\
         |   L  ) _.'\\                    ) _.'\\
         L    \\('`    \\                  ('`    \\
          ) _.'\\`-....'                   `-....'
         ('`    \\
          `-.___/  
", Color::Magenta)
}