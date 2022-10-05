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

#[derive(Clone, Debug)]
pub enum Message {
    INFO(String),
    QUESTION(String, usize),
    BRANCH(String, Vec<Branch>),
}

impl Message {
    pub fn new_info(info: String) -> Self {
        Self::INFO(info)
    }

    pub fn new_question(question: String, save_id: usize) -> Self {
        Self::QUESTION(question, save_id)
    }

    pub fn new_branch(question: String, options: Vec<Branch>) -> Self {
        Self::BRANCH(question, options)
    }
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub option: String,
    pub messages: Vec<Message>,
}

impl Branch {
    pub fn new(option: String, messages: Vec<Message>) -> Self {
        Self {
            option,
            messages,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    BrackOpen,              // {
    BrackClose,             // }
    LambdaOperator,         // =>
    Identifier(usize),      // ?0
    StringLiteral(String),  // ""
}

#[derive(Clone, Debug)]
pub struct Token {
    pub position: Position,
    pub token_type: TokenType,
}
