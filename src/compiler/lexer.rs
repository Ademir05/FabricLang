use crate::compiler::lexer::Token::*;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IntType,
    Identifier(String),
    Integer(i64),
    Assign,
    Plus,
    DoubleEqual,
    Semi,
    EOF,
}

pub struct LexicalError {
    message: String,
}

impl LexicalError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::EOF);
        }

        let c: char = self.peek();

        if c.is_alphabetic() {
            return self.read_identifier();
        }

        if c.is_ascii_digit() {
            return self.read_number();
        }

        match c {
            '=' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    Ok(Token::DoubleEqual)
                } else {
                    Ok(Token::Assign)
                }
            }
            _ => Err(LexicalError::new("Not implemented")),
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> Result<Token, LexicalError> {
        let start = self.position;
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        match identifier.as_str() {
            "int" => Ok(Token::IntType),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn read_number(&mut self) -> Result<Token, LexicalError> {
        let start = self.position;
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        let number_str: String = self.input[start..self.position].iter().collect();
        match number_str.parse::<i64>() {
            Ok(n) => Ok(Token::Integer(n)),
            Err(_) => Err(LexicalError::new("Invalid integer literal")),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.position];
        self.position += 1;
        c
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.input[self.position] == expected
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
}
