use std::collections::HashMap;
use pancurses::Window;

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

#[derive(Clone)]
pub struct Branch {
    pub option: String,
    pub messages: Vec<Message>,
}

