use pancurses::*;
use rpg_rs::*;
use std::{env, fs};

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: <file_name>");
        std::process::exit(1);
    }

    let source = if let Ok(m) = fs::read_to_string(args[1].clone()) {
        m
    } else {
        eprintln!("Error: Error while reading the file");
        std::process::exit(1);
    };

    match rpg_rs::compile(source) {
        Ok(messages) => {
            raw();

            let mut game = Game::new(initscr(), messages);
            game.window.keypad(true);
            start_game(&mut game);

            endwin();
            Ok(())
        }

        Err(error) => error.complain()
    }

}

