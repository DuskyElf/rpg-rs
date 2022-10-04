use std::fmt::Write;

use crate::models::*;

pub struct Lexer {
    source: String,
    index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn lex(source: String) -> Vec<TokenType> {
        let mut tokens: Vec<TokenType> = Vec::new();
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
                '{'  => tokens.push(TokenType::BrackOpen),
                '}'  => tokens.push(TokenType::BrackClose),
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

    fn lex_lambda_operator(&mut self) -> TokenType {
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

        TokenType::LambdaOperator
    }

    fn lex_identifier(&mut self) -> TokenType {
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
        TokenType::Identifier(number)
    }

    fn lex_strings(&mut self) -> TokenType {
        self.index += 1;
        self.column += 1;
        let mut result = String::new();

        if self.index >= self.source.len() {
            self.index -= 1;
            self.column -= 1;
            self.error("Missing end of '\"' (String literal)");
        }

        let start_line = self.line;
        let start_column = self.column;

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

        TokenType::StringLiteral(result)
    }

    fn error(&self, message: &str) {
        println!("Error: {}\nAt Line: {}, Column: {}",
            message, self.line, self.column
        );

        std::process::exit(1);
    }
}

