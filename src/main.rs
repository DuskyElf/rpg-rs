use pancurses::*;
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

