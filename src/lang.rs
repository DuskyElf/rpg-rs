use std::fmt::Write;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::models::*;
use TokenType::*;

pub struct Lexer {
    source: String,
    index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn lex(source: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut lexer = Lexer {
            source,
            index: 0,
            line: 1,
            column: 1,
        };

        loop {
            if lexer.index >= lexer.source.len() {
                break;
            }

            let current_char = lexer.source.chars().nth(lexer.index).unwrap();
            match current_char {
                '{'  => tokens.push(Token{
                    position: Position { line: lexer.line, column: lexer.column },
                    token_type: BrackOpen,
                }),
                '}'  => tokens.push(Token{
                    position: Position { line: lexer.line, column: lexer.column },
                    token_type: BrackClose,
                }),
                '='  => tokens.push(lexer.lex_lambda_operator()),
                '?'  => tokens.push(lexer.lex_identifier()),
                '"'  => tokens.push(lexer.lex_strings()),
                '\n' => {
                    lexer.line += 1;
                    lexer.column = 0;
                },
                ' ' | '\t' => (),
                _ => lexer.error("Invalid Syntax"),
            }
            lexer.index += 1;
            lexer.column += 1;
        }

        tokens
    }

    fn lex_lambda_operator(&mut self) -> Token {
        self.index += 1;
        self.column += 1;
        if self.index >= self.source.len() {
            self.index -= 1;
            self.column -= 1;
            self.error("Missing '>' after '=', for '=>' operator");
        }

        if self.source.chars().nth(self.index).unwrap() != '>' {
            self.error("Expected '>' after '=', for '=>' operator");
        }

        Token {
            position: Position { line: self.line, column: self.column - 1 },
            token_type: LambdaOperator,
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let start_column = self.column;
        self.index += 1;
        self.column += 1;

        if self.index >= self.source.len() {
            self.index -= 1;
            self.column -= 1;
            self.error("Missing identifier number after '?'");
        }

        let mut letter = self.source.chars().nth(self.index).unwrap();
        let mut number = String::new();
        while DIGITS.contains(&letter) {
            number.write_char(letter).unwrap();

            self.index += 1;
            self.column += 1;
            if self.index >= self.source.len() {
                break;
            }

            letter = self.source.chars().nth(self.index).unwrap();
        }
        self.index -= 1;
        self.column -= 1;

        if number.is_empty() {
            self.error("Expected identifier number after '?'")
        }

        let number: usize = number.parse().unwrap();
        Token {
            position: Position { line: self.line, column: start_column},
            token_type: Identifier(number),
        }
    }

    fn lex_strings(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;

        self.index += 1;
        self.column += 1;
        let mut result = String::new();

        if self.index >= self.source.len() {
            self.index -= 1;
            self.column -= 1;
            self.error("Missing end of '\"' (String literal)");
        }

        let mut letter = self.source.chars().nth(self.index).unwrap();
        while letter != '"' {
            if letter == '\n' {
                self.line += 1;
                self.column = 0;
            }

            result.write_char(letter).unwrap();

            self.index += 1;
            self.column += 1;
            if self.index >= self.source.len() {
                self.line = start_line;
                self.column = start_column;
                self.error("This '\"' (String literal) have no ending")
            }

            letter = self.source.chars().nth(self.index).unwrap();
        }

        Token {
            position: Position { line: start_line, column: start_column },
            token_type: StringLiteral(result),
        }
    }

    fn error(&self, message: &str) {
        println!("Error: {}\nAt Line: {}, Column: {}",
            message, self.line, self.column
        );

        std::process::exit(1);
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    curr_token: Token,
}

impl Parser {
    fn new(tokens: Peekable<IntoIter<Token>>) -> Self {
        Self {
            tokens,
            curr_token: Token {
                position: Position { line: 0, column: 0 },
                token_type: BrackOpen,
            },
        }
    }
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        let mut parser = Parser::new(tokens.into_iter().peekable());

        loop {
            parser.curr_token = match parser.tokens.next() {
                Some(token) => token,
                None => break,
            };

            messages.push(parser.handle_token())
        }

        messages
    }

    fn handle_token(&mut self) -> Message {
        match self.curr_token.token_type {
            StringLiteral(_) => self.parse_strings(),
            Identifier(_) => self.parse_identifier(),
            _ => {
                self.error("Expected Message");
                unreachable!()
            },
        }
    }

    fn parse_strings(&mut self) -> Message {
        let message = match self.curr_token.token_type.clone() {
            StringLiteral(m) => m,
            _ => unreachable!(),
        };

        if let Some(_) =
            self.tokens.next_if(|x| x.token_type == BrackOpen) {
                self.parse_branch(message)
        }
        else {
            Message::new_info(message)
        }

    }

    fn parse_identifier(&mut self) -> Message {
        let save_id = match self.curr_token.token_type {
            Identifier(m) => m,
            _ => unreachable!(),
        };

        let question_token = if let Some(token) = self.tokens.next() {
            self.curr_token = token.clone();
            token
        } else {
            self.error("Missing question (StringLiteral)");
            unreachable!()
        };
        let question = if let StringLiteral(question) = question_token.token_type {
            question
        } else {
            self.error("Expected question (StringLiteral)");
            unreachable!()
        };

        Message::new_question(question, save_id)
    }

    fn parse_branch(&mut self, question: String) -> Message {
        let mut branches: Vec<Branch> = Vec::new();
        while let None = self.tokens.next_if(|x| x.token_type == BrackClose) {
            let branch_name_token = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                self.error("Missing '}' (Branch ending)");
                unreachable!()
            };

            let branch_name = if let StringLiteral(branch_name) = branch_name_token.token_type {
                branch_name
            } else {
                self.error("Expected Branch Node");
                unreachable!()
            };

            let node_delaration = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                self.error("Missing '=>' (Branch Node declaration)");
                unreachable!()
            };
            if node_delaration.token_type != LambdaOperator {
                self.error("Expected '=>' (Branch Node declaration)");
            };

            let node_starting = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                self.error("Missing '{' (Branch Node starting)");
                unreachable!();
            };
            if node_starting.token_type != BrackOpen {
                self.error("Expected '{' (Branch Node starting)");
            };

            let mut branch_node_block: Vec<Message> = Vec::new();
            while let None = self.tokens.next_if(|x| x.token_type == BrackClose) {
                if let Some(token) = self.tokens.next() {
                    self.curr_token = token.clone();
                    token
                } else {
                    self.error("Missing '}' (Branch Node ending)");
                    unreachable!()
                };

                branch_node_block.push(self.handle_token());
            }

            branches.push(Branch::new(branch_name, branch_node_block))
        }

        Message::new_branch(question, branches)
    }

    fn error(&self, message: &str) {
        println!("Error: {}\nAt Line: {}, Column: {}",
            message, self.curr_token.position.line,
            self.curr_token.position.column
        );

        std::process::exit(1);
    }
}
