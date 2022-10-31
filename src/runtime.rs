pub use crate::*;
use models::DIGITS;

pub fn start_game(game: &mut Game) {
    let byte_code = game.byte_code.clone();
    let mut iptr = 0;
    loop {
        let is_end = run(byte_code[iptr], &mut iptr, game);
        if is_end {
            break;
        }
    }
}

fn run(op_code: OpCode, iptr: &mut usize, game: &mut Game) -> bool {
    // Decoding instructions to different functions
    match op_code {
        OpCode::NOP => (),
        OpCode::END => return true,
        OpCode::JMP(ptr) => {
            *iptr = ptr;
            return false;
        },
        OpCode::TELL(info) => msg_tell(info, game),
        OpCode::ASK(question, id) => msg_question(question, id, game),
        OpCode::BRANCH(question, branches) =>
            return run(msg_branch(question, branches, game), iptr, game),
    }

    *iptr += 1;
    false
}

fn msg_tell(info: String, game: &Game) {
    tell_info(info, game);
    game.window.addstr("\n\nPress any key to continue");
    game.window.refresh();

    curs_set(0);
    noecho();
    game.window.getch();
    echo();
    curs_set(1);
}

fn msg_question(question: String, id: Option<usize>, game: &mut Game) {
    tell_info(question, game);

    game.window.addstr("\n\n>");
    let mut responce = String::new();
    scan(&game.window, &mut responce);

    if let Some(id) = id {
        game.states.insert(id, responce);
    }
}

fn msg_branch(question: String, branches: Vec<Branch>, game: &mut Game) -> OpCode {
    tell_info(question, game);
    game.window.addstr("\n\n");

    let branch = branch_selection(branches, game);

    branch.handler
}

fn tell_info(info: String, game: &Game) {
    game.window.clear();
    game.window.mv(0, 0);

    let viewable = parse(info, game);

    for letter in viewable.chars() {
        game.window.addch(letter as u32);
        thread::sleep(time::Duration::from_millis(15));
        game.window.refresh();
    }
}

// Parsing StringLiteral to find an identifer reference
fn parse(info: String, game: &Game) -> String {
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

// Parsing identifer reference inside a StringLiteral
// And replacing it with it's value from game.states (runtime identifer pool)
fn handle_states(i: &mut usize, info: String, game: &Game) -> String {
    *i += 1;
    let mut result = String::new();
    if *i < info.len() {
        let mut letter = info.chars().nth(*i).unwrap();
        let mut number = String::new();
        while DIGITS.contains(&letter) {
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

fn branch_selection(branches: Vec<Branch>, game: &Game) -> &Branch {
    curs_set(0);
    noecho();
    let mut selection = 0;
    let y = game.window.get_cur_y();
    let x = game.window.get_cur_x();

    // Looping through all choices in the branch
    // And rendering them in responce to user
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

// Simulating stdin
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

