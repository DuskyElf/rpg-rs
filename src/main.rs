use pancurses::*;
use std::{thread, time};

mod models;
use models::*;

fn main() {
    raw();

    let mut game = Game::new(initscr(), vec![
        Message::INFO("Hello, World! This is just so awesome!!!".to_string()),
        Message::QUESTION("What is your name?".to_string(), 0),
    ]);

    start_game(&mut game);
    game.window.getch();

    endwin();
}
fn start_game(game: &mut Game) {
    for message in game.messages.clone() {
        handel_message(&message, game);
    }
}

fn handel_message(message: &Message, game: &mut Game) {
    match message {
        Message::INFO(info) => info_message(info, game),
        Message::QUESTION(question, id) => question_message(question, id, game),
        // Message::BRANCH(question, branches) => branch_message(question, branches),
        _ => (),
    }
}

fn info_message(info: &String, game: &Game) {
    tell_info(info, game);
    game.window.addstr("\n\nPress any key to continue");
    game.window.refresh();

    curs_set(0);
    noecho();
    game.window.getch();
    echo();
    curs_set(1);
}

fn question_message(question: &String, id: &usize, game: &mut Game) {
    tell_info(question, game);

    game.window.addstr("\n\n>");
    let responce = String::new();
    // scanf(&game.window, &mut responce);

    game.window.addstr(&responce);
}

fn tell_info(info: &String, game: &Game) {
    game.window.clear();
    game.window.mv(0, 0);
    for char in info.chars() {
        game.window.addch(char as u32);
        thread::sleep(time::Duration::from_millis(15));
        game.window.refresh();
    }
}

