use crate::compiler::lexer::Token::*;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Types
    IntType,
    BigIntType,
    FloatType,
    DoubleType,
    StringType,
    BoolType,
    CharType,
    VoidType,

    // Literals
    Integer(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    CharLiteral(char),

    // Identifiers
    Identifier(String),

    // Aritmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Logical Operators
    And,
    Or,

    // Parentheses
    LeftParen,
    RightParen,

    // Brackets
    LeftBracket,
    RightBracket,

    // Braces
    LeftBrace,
    RightBrace,

    // Commas
    Comma,

    // Comparison Operators
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Not,

    // Control Structures
    If,
    Else,
    While,
    For,
    Switch,
    Case,
    Default,

    // Functions
    Function,
    Return,

    Assign,
    Semi,
    EOF,
}

#[derive(Debug)]
struct LexicalError {
    message: String,
}

impl LexicalError {
    fn new(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: format!("{} (at {}:{})", message, line, column),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn next_token(&mut self) -> Result<Token, LexicalError> {
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
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '{' => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::RightBrace)
            }
            '[' => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightBracket)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }

            // Operadores lógicos (&& y ||)
            '&' => {
                self.advance();
                if self.match_next('&') {
                    self.advance();
                    Ok(Token::And)
                } else {
                    let msg = format!("Expected '&' after '&'");
                    return Err(LexicalError::new(&msg, self.line, self.column));
                }
            }
            '|' => {
                self.advance();
                if self.match_next('|') {
                    self.advance();
                    Ok(Token::Or)
                } else {
                    let msg = format!("Expected '|' after '|'");
                    return Err(LexicalError::new(&msg, self.line, self.column));
                }
            }

            // Assign and Comparison Operators
            '=' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(Token::Equal);
                } else {
                    return Ok(Token::Assign);
                }
            }
            '<' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(Token::LessEqual);
                } else {
                    return Ok(Token::Less);
                }
            }
            '>' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(Token::GreaterEqual);
                } else {
                    return Ok(Token::Greater);
                }
            }
            '!' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(Token::NotEqual);
                } else {
                    return Ok(Token::Not);
                }
            }

            // Aritmetic Operators
            '+' => {
                self.advance();
                return Ok(Token::Plus);
            }
            '-' => {
                self.advance();
                return Ok(Token::Minus);
            }
            '*' => {
                self.advance();
                return Ok(Token::Multiply);
            }
            '/' => {
                self.advance();
                if self.match_next('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    self.next_token()
                } else {
                    Ok(Token::Divide)
                }
            }
            '%' => {
                self.advance();
                return Ok(Token::Modulo);
            }
            '^' => {
                self.advance();
                return Ok(Token::Power);
            }

            // Other Operators
            ';' => {
                self.advance();
                return Ok(Token::Semi);
            }
            '"' => return self.read_string(),
            '\'' => return self.read_char(),

            // Not supported
            _ => {
                let msg = format!("Unexpected character '{}' at position {}", c, self.position);
                return Err(LexicalError::new(&msg, self.line, self.column));
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.input[self.position] == expected
    }

    fn read_identifier(&mut self) -> Result<Token, LexicalError> {
        let start: usize = self.position;
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        match identifier.as_str() {
            "int" => Ok(Token::IntType),
            "bigint" => Ok(Token::BigIntType),
            "string" => Ok(Token::StringType),
            "bool" => Ok(Token::BoolType),
            "float" => Ok(Token::FloatType),
            "double" => Ok(Token::DoubleType),
            "char" => Ok(Token::CharType),
            "void" => Ok(Token::VoidType),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "while" => Ok(Token::While),
            "for" => Ok(Token::For),
            "switch" => Ok(Token::Switch),
            "case" => Ok(Token::Case),
            "default" => Ok(Token::Default),
            "function" => Ok(Token::Function),
            "true" => Ok(Token::BoolLiteral(true)),
            "false" => Ok(Token::BoolLiteral(false)),
            "return" => Ok(Token::Return),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn read_char(&mut self) -> Result<Token, LexicalError> {
        self.advance();
        if self.is_at_end() {
            let msg = format!("Unterminated character literal");
            return Err(LexicalError::new(&msg, self.line, self.column));
        }
        let content: char = self.advance();

        if self.peek() != '\'' {
            let msg = format!("Invalid character literal");
            return Err(LexicalError::new(&msg, self.line, self.column));
        }
        self.advance();
        return Ok(Token::CharLiteral(content));
    }

    fn read_string(&mut self) -> Result<Token, LexicalError> {
        self.advance();
        let start: usize = self.position;
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        if self.is_at_end() {
            let msg = format!("Unterminated string literal");
            return Err(LexicalError::new(&msg, self.line, self.column));
        }
        let content: String = self.input[start..self.position].iter().collect();
        self.advance();
        return Ok(Token::StringLiteral(content));
    }

    fn read_number(&mut self) -> Result<Token, LexicalError> {
        let start: usize = self.position;
        let mut is_float = false;
        while !self.is_at_end() && self.input[self.position].is_ascii_digit() {
            self.advance();
        }
        if !self.is_at_end() && self.peek() == '.' {
            is_float = true;
            self.advance();
            if !self.peek().is_ascii_digit() {
                let msg = format!("Expected digit after decimal point");
                return Err(LexicalError::new(&msg, self.line, self.column));
            }
            while !self.is_at_end() && self.input[self.position].is_ascii_digit() {
                self.advance();
            }
        }
        let literal: String = self.input[start..self.position].iter().collect();

        if !self.is_at_end() && self.input[self.position].is_alphabetic() {
            return Err(LexicalError::new(format!("Invalid number literal at position {}", self.position).as_str(), self.line, self.column));
        }

        if is_float {
            match literal.parse::<f64>() {
                Ok(n) => Ok(Token::FloatLiteral(n)),
                Err(_) => Err(LexicalError::new("Invalid float literal", self.line, self.column)),
            }
        } else {
            match literal.parse::<i64>() {
                Ok(n) => Ok(Token::Integer(n)),
                Err(_) => Err(LexicalError::new("Invalid integer literal", self.line, self.column)),
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.position];
        self.position += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token == Token::EOF;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Lexical Error: {}", e.message);
                    std::process::exit(1);
                }
            }
        }
        tokens
    }
}
