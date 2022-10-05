use pancurses::*;
use rpg_rs::*;
use std::{env, fs};

fn main() {
    raw();

    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        println!("Usage: <file_name>")
    }

    let source = fs::read_to_string(args[1].clone()).expect("Can't read the file");

    let messages = rpg_rs::compile(source);

    let mut game = Game::new(initscr(), messages);

    game.window.keypad(true);

    start_game(&mut game);

    endwin();
}

