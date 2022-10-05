use pancurses::*;
use rpg_rs::*;
use std::{env, fs};

fn main1() {
    raw();

    let mut game = Game::new(initscr(), vec![]);

    game.window.keypad(true);

    start_game(&mut game);

    endwin();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        println!("Usage: <file_name>")
    }

    let source = fs::read_to_string(args[1].clone()).expect("Can't read the file");

    let tokens = rpg_rs::Lexer::lex(source);
    let messages = rpg_rs::Parser::parse(tokens);
    println!("{:#?}", messages);
}

