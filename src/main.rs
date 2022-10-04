use pancurses::*;
use rpg_rs::*;
use std::{env, fs};

fn main1() {
    raw();

    let mut game = Game::new(initscr(), vec![
        Message::new_info("Hello, World! This is just so awesome!!!"),
        Message::new_branch("Which Branch?", vec![
            Branch::new("first", vec![
                Message::new_question("What is your name?", 0),
                Message::new_question("What is your age?", 1),
                Message::new_info("Hi $0, you are $1 years old!"),
            ]),

            Branch::new("second", vec![
                Message::new_branch("Are you 18+ ?", vec![
                    Branch::new("yes", vec![
                        Message::new_info("Whooo, You can vote")
                    ]),
                    Branch::new("no", vec![
                        Message::new_info("Sorry, you can't vote")
                    ]),
                ])
            ]),

        ]),
    ]);

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

    println!("{:#?}", rpg_rs::Lexer::lex(source));
}

