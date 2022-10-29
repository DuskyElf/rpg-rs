use std::{fmt::Write, vec, collections::{HashMap, HashSet}};

use crate::models::*;
use TokenType::*;
use ErrorType::*;

// The tokenizer
impl Lexer {
    fn lex(source: String) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        
        // Global variables for different parts of the lexer
        let mut lexer = Lexer {
            source,
            index: 0,
            line: 1,
            column: 1,
        };

        // Iterates through each char in the rpg_source
        loop {
            if lexer.index >= lexer.source.len() {
                break;
            }

            // AskOp,                  // ?
            // TellOp,                 // -
            // ParOpen,                // (
            // ParClose,               // )
            // BranchOp,               // #
            // LambdaOp,               // =>
            // BrackOpen,              // {
            // BrackClose,             // }
            // AssignmentOp,           // :=
            // Identifier(String),     // <a-zA-Z0-9>
            // StringLiteral(String),  // ""

            let current_char = lexer.source.chars().nth(lexer.index).unwrap();
            let position = Position { line: lexer.line, column: lexer.column };
            match current_char {
                '?'  => tokens.push(Token{
                    position, token_type: AskOp,
                }),

                '-'  => tokens.push(Token{
                    position, token_type: TellOp,
                }),

                '('  => tokens.push(Token{
                    position, token_type: ParOpen,
                }),

                ')'  => tokens.push(Token{
                    position, token_type: ParClose,
                }),

                '#'  => tokens.push(Token{
                    position, token_type: BranchOp,
                }),

                '='  => tokens.push(lexer.lex_lambda_op()?),

                '{'  => tokens.push(Token{
                    position, token_type: BrackOpen,
                }),

                '}'  => tokens.push(Token{
                    position, token_type: BrackClose,
                }),

                ':'  => tokens.push(lexer.lex_assignment_op()?),
                
                'a'..='z' | 'A'..='Z' | '0'..='9' => tokens.push(lexer.lex_identifier()?),

                '"'  => tokens.push(lexer.lex_string_literal()?),

                '\n' => {
                    lexer.line += 1;
                    lexer.column = 0;
                },

                ' ' | '\t' => (), // Ignoring white spaces

                _ => return Err(Error::lex_error(
                    // TODO: InvalidChar Error
                    InvalidSyntax, &lexer
                ))
            }
            lexer.index += 1;
            lexer.column += 1;
        }

        Ok(tokens)
    }

    // `=>`
    fn lex_lambda_op(&mut self) -> Result<Token, Error> {
        self.index += 1;
        self.column += 1;
        if self.index >= self.source.len() {
            // For error to point at the right location
            self.index -= 1;
            self.column -= 1;

            return Err(Error::lex_error(
                Missing("'>' after '=', for '=>' operator".to_string()),
                self
            ))
        }

        if self.source.chars().nth(self.index).unwrap() != '>' {
            return Err(Error::lex_error(
                Expected("'>' after '=', for '=>' operator".to_string()),
                self
            ))
        }

        Ok(Token {
            position: Position { line: self.line, column: self.column - 1 },
            token_type: LambdaOp,
        })
    }

    // `:=`
    fn lex_assignment_op(&mut self) -> Result<Token, Error> {
        self.index += 1;
        self.column += 1;
        if self.index >= self.source.len() {
            // For error to point at the right location
            self.index -= 1;
            self.column -= 1;

            return Err(Error::lex_error(
                Missing("'=' after ':', for AssignmentOp (':=')".to_string()),
                self
            ))
        }

        if self.source.chars().nth(self.index).unwrap() != '=' {
            return Err(Error::lex_error(
                Expected("'=' after ':', for AssignmentOp (':=')".to_string()),
                self
            ))
        }

        Ok(Token {
            position: Position { line: self.line, column: self.column - 1 },
            token_type: AssignmentOp,
        })
    }

    // `*<a-zA-Z0-9>`
    fn lex_identifier(&mut self) -> Result<Token, Error> {
        let start_column = self.column;
        let identifier = String::new();

        let mut letter = self.source.chars().nth(self.index).unwrap();
        while
            ('a'..='z').contains(&letter) |
            ('A'..='Z').contains(&letter) |
            ('0'..='9').contains(&letter)
        {
            identifier.write_char(letter).unwrap();

            self.index += 1;
            self.column += 1;
            if self.index >= self.source.len() { break; }
            let mut letter = self.source.chars().nth(self.index).unwrap();
        }
        Ok(Token {
            position: Position { line: self.line, column: start_column},
            token_type: Identifier(identifier),
        })
    }

    // `"*<.-">"`
    fn lex_string_literal(&mut self) -> Result<Token, Error> {
        let start_line = self.line;
        let start_column = self.column;

        self.index += 1;
        self.column += 1;
        let mut result = String::new();

        if self.index >= self.source.len() {
            // For error to point at the right location
            self.index -= 1;
            self.column -= 1;
            return Err(Error::lex_error(
                Missing("end of '\"' (String literal)".to_string()),
                self
            ))
        }

        let mut letter = self.source.chars().nth(self.index).unwrap();
        while letter != '"' {
            // Keeping track of line, column for error messages
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
                return Err(Error::lex_error(
                    Missing("This '\"' (String literal) have no ending".to_string()),
                    self
                ))
            }

            letter = self.source.chars().nth(self.index).unwrap();
        }

        Ok(Token {
            position: Position { line: start_line, column: start_column },
            token_type: StringLiteral(result),
        })
    }
}

enum ExprType{
    Block,
    Value,
}

// TODO: Implement parser according to the new grammar
impl Parser {
    fn new(tokens: ParseableTokens) -> Self {
        // Global variables for different parts of the Parser
        Self {
            tokens,
            byte_code: vec![],
            curr_token: Token {
                position: Position { line: 0, column: 0 },
                token_type: BrackOpen,
            },
            value_identifiers: HashMap::new(),
            block_identifiers: HashMap::new(),
        }
    }

    fn parse(tokens: Vec<Token>) -> Result<Vec<OpCode>, Error> {
        let mut parser = Parser::new(tokens.into_iter().peekable());

        if let Some(token) = parser.tokens.next() {
            parser.curr_token = token;
            parser.source()?;
        }

        Ok(parser.byte_code)
    }

    fn source(&mut self) -> Result<(), Error> {
        loop {
            if let Some(statement_token) = self.tokens.next() {
                self.curr_token = statement_token;
                self.statement()?;
            }
            else {
                break;
            }
        }

        Ok(())

    }

    fn statement(&mut self) -> Result<(), Error> {
        if let TokenType::TellOp = self.curr_token.token_type {
            self.tell()?;
            return Ok(());
        }
        if let TokenType::BranchOp = self.curr_token.token_type {
            self.branch()?;
            return Ok(());
        }

        if let TokenType::Identifier(identifier) = self.curr_token.token_type {
            if let Some(_) = self.tokens.next_if(|x| x.token_type == AssignmentOp) {
                if let Some(statement_token) = self.tokens.next() {
                    self.curr_token = statement_token;

                    let value_spot = self.value_identifiers.len();
                    let block_spot = self.block_identifiers.len();
                    match self.expr(value_spot, block_spot)? {
                        ExprType::Value => {
                            self.value_identifiers.insert(identifier, value_spot);
                        },
                        ExprType::Block =>{
                            self.block_identifiers.insert(identifier, block_spot);
                        }
                    };
                    return Ok(());
                }
                else {
                    return Err(Error::parse_error(
                        Missing("expression for variable assignment".to_string()),
                        self
                    ));
                }
            }
        }

        self.expr(usize::MAX, usize::MAX)?;
        Ok(())
    }

    fn expr(&mut self, value_spot: usize, block_spot: usize) -> Result<ExprType, Error> {
        todo!()
    }

    fn ask() {
        todo!()
    }

    fn tell(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn branch(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn branches() {
        todo!()
    }

    fn block() {
        todo!()
    }

    fn arguments() {
        todo!()
    }

    // StringLiteral
    // Identifer
    fn handle_token(&mut self) -> Result<OpCode, Error> {
        match self.curr_token.token_type {
            StringLiteral(_) => self.parse_strings(),
            Identifier(_) => self.parse_identifier(),
            _ => {
                return Err(Error::parse_error(
                    Expected("Message".to_string()),
                    self
                ))
            },
        }
    }

    // StringLiteral BrackOpen
    // StringLiteral
    fn parse_strings(&mut self) -> Result<OpCode, Error> {
        self.validate_string()?;
        let message = match self.curr_token.token_type.clone() {
            StringLiteral(m) => m,
            _ => unreachable!(),
        };

        if let Some(_) =
            self.tokens.next_if(|x| x.token_type == BrackOpen) {
                self.parse_branch(message);
        }
        else {
        }
        unreachable!()

    }

    // Identifer StringLiteral
    fn parse_identifier(&mut self) -> Result<OpCode, Error> {
        let save_id = match self.curr_token.token_type {
            Identifier(m) => m,
            _ => unreachable!(),
        };

        let question_token = if let Some(token) = self.tokens.next() {
            self.curr_token = token.clone();
            token
        } else {
            return Err(Error::parse_error(
                Missing("question (StringLiteral)".to_string()),
                self
            ))
        };
        let question = if let StringLiteral(question) = question_token.token_type {
            self.validate_string()?;
            question
        } else {
            return Err(Error::parse_error(
                Expected("question (StringLiteral)".to_string()),
                self
            ))
        };

        unreachable!()
    }

    // BrackOpen +(StringLiteral LambdaOperator BrackOpen *. BrackClose) BrackClose
    fn parse_branch(&mut self, question: String) -> Result<OpCode, Error> {
        let mut branches: Vec<Branch> = Vec::new();

        // Looping through all choices of the branch
        while let None = self.tokens.next_if(|x| x.token_type == BrackClose) {
            let branch_name_token = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                return Err(Error::parse_error(
                    Missing("'}' (Branch ending)".to_string()),
                    self
                ))
            };

            let branch_name = if let StringLiteral(branch_name) = branch_name_token.token_type {
                self.validate_string()?;
                branch_name
            } else {
                return Err(Error::parse_error(
                    Expected("Branch Node".to_string()),
                    self
                ))
            };

            let node_delaration = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                return Err(Error::parse_error(
                    Missing("'=>' (Branch Node declaration)".to_string()),
                    self
                ))
            };
                return Err(Error::parse_error(
                    Expected("'=>' (Branch Node declaration)".to_string()),
                    self
                ))

            let node_starting = if let Some(token) = self.tokens.next() {
                self.curr_token = token.clone();
                token
            } else {
                return Err(Error::parse_error(
                    Missing("'{' (Branch Node starting)".to_string()),
                    self
                ))
            };
            if node_starting.token_type != BrackOpen {
                return Err(Error::parse_error(
                    Expected("'{' (Branch Node starting)".to_string()),
                    self
                ))
            };

            let mut branch_node_block: Vec<OpCode> = Vec::new();
            let tmp = self.value_identifiers.clone();

            // Looping through all tokens inside the branch body
            // to recursively parse them all
            while let None = self.tokens.next_if(|x| x.token_type == BrackClose) {
                if let Some(token) = self.tokens.next() {
                    self.curr_token = token.clone();
                    token
                } else {
                    return Err(Error::parse_error(
                        Missing("'}' (Branch Node ending)".to_string()),
                        self
                    ))
                };

                branch_node_block.push(self.handle_token()?);
            }
            self.value_identifiers = tmp;

            branches.push(Branch::new(branch_name, branch_node_block))
        }

        Ok(OpCode::new_branch(question, branches))
    }

    // Checking if StringLiteral have valid identifier references
    // Identifer reference be -> `$<*digit>`
    fn validate_string(&self) -> Result<(), Error> {
        let info = if let StringLiteral(m) =
            self.curr_token.token_type.clone() { m } else { unreachable!() };

        let mut i = 0;
        while i < info.len() {
            let mut letter = info.chars().nth(i).unwrap();
            if letter == '$' {
                i += 1;
                if i >= info.len() { break; }
                letter = info.chars().nth(i).unwrap();

                let mut number = String::new();
                while DIGITS.contains(&letter) {
                    number.write_char(letter).unwrap();
                    i += 1;
                    if i >= info.len() { break; }
                    letter = info.chars().nth(i).unwrap();
                }

                if !number.is_empty() {
                    let number: usize = number.parse().unwrap();
                    if !self.value_identifiers.contains(&number) {
                        //"Identifer '{}' used in StringLiteral without delaration",
                        return Err(Error::parse_error(
                            InvalidIdentifier(number),
                            self
                        ))
                    }
                }
            }

            i += 1;
        }
        Ok(())
    }
}

pub fn compile(source: String) -> Result<Vec<OpCode>, Error> {
    let tokens = Lexer::lex(source)?;
    let messages = Parser::parse(tokens)?;

    Ok(messages)
}
