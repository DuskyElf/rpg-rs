use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Peekable;
use std::vec::IntoIter;

use pancurses::Window;

pub static DIGITS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub struct Game {
    pub window: Window,
    pub byte_code: Vec<OpCode>,
    pub states: HashMap<usize, String>,
}

impl Game {
    pub fn new(window: Window, byte_code: Vec<OpCode>) -> Self {
        Self {
            window,
            byte_code,
            states: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OpCode {
    NOP,
    END,
    JMP(usize),
    TELL(String),
    ASK(String, Option<usize>),
    BRANCH(String, Vec<Branch>),
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub option: String,
    pub handler: OpCode,
}

impl Branch {
    pub fn new(option: String, handler: OpCode) -> Self {
        Self {
            option,
            handler,
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
    AskOp,                  // ?
    TellOp,                 // -
    ParOpen,                // (
    ParClose,               // )
    BranchOp,               // #
    LambdaOp,               // =>
    BrackOpen,              // {
    BrackClose,             // }
    AssignmentOp,           // :=
    Identifier(String),     // <a-zA-Z0-9>
    StringLiteral(String),  // ""
}

pub struct Lexer {
    pub source: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub position: Position,
    pub token_type: TokenType,
}

pub type ParseableTokens = Peekable<IntoIter<Token>>;
pub struct Parser {
    pub tokens: ParseableTokens,
    pub byte_code: Vec<OpCode>,
    pub curr_token: Token,
    pub value_identifiers: HashMap<String, usize>,
    pub block_identifiers: HashMap<String, usize>,
}

pub enum ErrorType {
    InvalidSyntax,
    Missing(String),
    Expected(String),
    InvalidIdentifier(usize),
}

pub struct Error {
    error_type: ErrorType,
    line: usize,
    column: usize,
}

impl Error {
    pub fn lex_error(error_type: ErrorType, lexer: &Lexer) -> Self {
        Self {
            error_type,
            line: lexer.line,
            column: lexer.column,
        }
    }

    pub fn parse_error(error_type: ErrorType, parser: &Parser) -> Self {
        Self {
            error_type,
            line: parser.curr_token.position.line,
            column: parser.curr_token.position.column,
        }
    }

    pub fn complain(self) -> Result<(), i32> {
        match self.error_type {
            ErrorType::InvalidSyntax =>{
                eprintln!(
                    "Error: Invalid Syntax\nAt line: {}, column: {}",
                    self.line, self.column
                );
                Err(-1)
            },

            ErrorType::Missing(error) => {
                eprintln!(
                    "Error: Missing {}\nAt line: {}, column: {}",
                    error, self.line, self.column
                );
                Err(40)
            },

            ErrorType::Expected(error) => {
                eprintln!(
                    "Error: Expected {}\nAt line: {}, column: {}",
                    error, self.line, self.column
                );
                Err(41)
            },

            ErrorType::InvalidIdentifier(identifier) => {
                eprintln!(
                    "Error: Identifer '{}' used in StringLiteral without delaration
At line: {}, column: {}",
                    identifier, self.line, self.column
                );
                Err(42)
            },
        }
    }
}
