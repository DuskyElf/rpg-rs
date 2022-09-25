use pancurses::*;
use std::{thread, time, fmt::Write};

mod models;
use models::*;

fn main() {
    raw();

    let mut game = Game::new(initscr(), vec![
        Message::INFO("Hello, World! This is just so awesome!!!".to_string()),
        Message::QUESTION("What is your name?".to_string(), 0),
        Message::QUESTION("What is your age?".to_string(), 1),
        Message::INFO("Hi $0, and you are $1 years old!".to_string()),
    ]);

    start_game(&mut game);

    endwin();
}
fn start_game(game: &mut Game) {
    for message in game.messages.clone() {
        handle_message(&message, game);
    }
}

fn handle_message(message: &Message, game: &mut Game) {
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
    let mut responce = String::new();
    scan(&game.window, &mut responce);

    game.states.insert(*id, responce);
}

fn tell_info(info: &String, game: &Game) {
    game.window.clear();
    game.window.mv(0, 0);

    let viewable = parse(info, game);

    for letter in viewable.chars() {
        game.window.addch(letter as u32);
        thread::sleep(time::Duration::from_millis(15));
        game.window.refresh();
    }
}

fn parse(info: &String, game: &Game) -> String {
    let mut i = 0;
    let mut result = String::new();
    while i < info.len() {
        let letter = info.chars().nth(i).unwrap();
        if letter == '$' {
            result += &handle_states(&mut i, info, game);
        }
        else {
            result.write_char(letter).unwrap();
            i += 1;
        }
    }
    result
}

fn handle_states(i: &mut usize, info: &String, game: &Game) -> String {
    *i += 1;
    let mut result = String::new();
    if *i < info.len() {
        let mut letter = info.chars().nth(*i).unwrap();
        let mut number = String::new();
        while ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&letter) {
            number.write_char(letter).unwrap();
            *i += 1;
            if *i >= info.len() { break; }
            letter = info.chars().nth(*i).unwrap();
        }
        if !number.is_empty() {
            let number: usize = number.parse().unwrap();
            result += &game.states[&number];
        }
        else {
            result.write_char(info.chars().nth(*i-1).unwrap()).unwrap();
        }

        if *i >= info.len() { return result; }
    }
    else {
        result.write_char(info.chars().nth(*i-1).unwrap()).unwrap();
    }
    result
}

fn scan(window: &Window, buffer: &mut String) {
    noecho();
    loop {
        if let Input::Character(read) = window.getch().unwrap() {
            if read == '\n' {
                echo();
                break;
            }

            // backspace
            if read == '\x7f' {
                if buffer.len() != 0 {
                    buffer.pop();
                    window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                    window.delch();
                }
                continue;
            }

            window.addch(read);
            buffer.push(read);
        }
    }
}

