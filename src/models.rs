use std::collections::HashMap;
use pancurses::Window;

pub static DIGITS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub struct Game {
    pub window: Window,
    pub messages: Vec<Message>,
    pub states: HashMap<usize, String>,
}

impl Game {
    pub fn new(window: Window, messages: Vec<Message>) -> Self {
        Self {
            window,
            messages,
            states: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    INFO(String),
    QUESTION(String, usize),
    BRANCH(String, Vec<Branch>),
}

impl Message {
    pub fn new_info(info: &str) -> Self {
        Self::INFO(info.to_string())
    }

    pub fn new_question(question: &str, save_id: usize) -> Self {
        Self::QUESTION(question.to_string(), save_id)
    }

    pub fn new_branch(question: &str, options: Vec<Branch>) -> Self {
        Self::BRANCH(question.to_string(), options)
    }
}

#[derive(Clone)]
pub struct Branch {
    pub option: String,
    pub messages: Vec<Message>,
}

impl Branch {
    pub fn new(option: &str, messages: Vec<Message>) -> Self {
        Self {
            option: option.to_string(),
            messages,
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum TokenType {
    BrackOpen(Position),              // {
    BrackClose(Position),             // }
    LambdaOperator(Position),         // =>
    Identifier(Position, usize),      // ?0
    StringLiteral(Position, String),  // ""
}
