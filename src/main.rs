use pancurses::*;
use std::{thread, time, fmt::Write};

use rpg_rs::*;

fn main() {
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

fn start_game(game: &mut Game) {
    for message in game.messages.clone() {
        handle_message(&message, game);
    }
}

fn handle_message(message: &Message, game: &mut Game) {
    match message {
        Message::INFO(info) => info_message(info, game),
        Message::QUESTION(question, id) => question_message(question, id, game),
        Message::BRANCH(question, branches) => branch_message(question, branches, game),
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

fn branch_message(question: &String, branches: &Vec<Branch>, game: &mut Game) {
    tell_info(question, game);
    game.window.addstr("\n\n");

    let branch = branch_selection(branches, game);

    for message in branch.messages.iter() {
        handle_message(&message, game);
    }
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
    }
    else {
        result.write_char(info.chars().nth(*i-1).unwrap()).unwrap();
    }
    result
}

fn branch_selection<'a>(branches: &'a Vec<Branch>, game: &Game) -> &'a Branch {
    curs_set(0);
    noecho();
    let mut selection = 0;
    let y = game.window.get_cur_y();
    let x = game.window.get_cur_x();
    loop {
        game.window.mv(y, x);
        for (i, branch) in branches.iter().enumerate() {
            if i == selection {
                game.window.addstr(format!(">[ {} ]\n", branch.option));
            } else {
                game.window.addstr(format!("   {}  \n", branch.option));
            }
        }

        match game.window.getch().unwrap() {
            Input::Character('\n') => break, // Enter / Return
            Input::KeyDown => selection += 1,
            Input::KeyUp => if selection != 0 {selection -= 1},
            _ => (),
        }

        if selection >= branches.len() {
            selection = branches.len() - 1;
        }
    
    }
    echo();
    curs_set(1);
    branches.get(selection).unwrap()
}

fn scan(window: &Window, buffer: &mut String) {
    noecho();
    loop {
        match window.getch().unwrap() {
            // Enter / Return
            Input::Character('\n') => {
                echo();
                break;
            },

            Input::KeyBackspace => {
                if buffer.len() != 0 {
                    buffer.pop();
                    window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                    window.delch();
                }
                continue;
            },

            Input::Character(read) => {
                window.addch(read);
                buffer.push(read);
            },

            _ => (),
        }
    }
}

