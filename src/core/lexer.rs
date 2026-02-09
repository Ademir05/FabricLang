use std::collections::HashMap;

use crate::persistence::models::SyntaxConfig;
use crate::core::token::TokenData;
use crate::core::token::Token;

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
    pub keywords_lookup: HashMap<String, Token>,
}

impl Lexer {
    fn emit(&self, kind: Token, start_col: usize) -> TokenData {
        TokenData::new(kind, self.line, start_col)
    }

    pub fn new(input: &str, config: &SyntaxConfig) -> Self {
        let mut lookup = HashMap::new();
        lookup.insert(config.types.int_type.clone(), Token::IntType);
        lookup.insert(config.types.big_int_type.clone(), Token::BigIntType);
        lookup.insert(config.types.string_type.clone(), Token::StringType);
        lookup.insert(config.types.bool_type.clone(), Token::BoolType);
        lookup.insert(config.types.float_type.clone(), Token::FloatType);
        lookup.insert(config.types.double_type.clone(), Token::DoubleType);
        lookup.insert(config.types.void_type.clone(), Token::VoidType);
        lookup.insert(config.types.char_type.clone(), Token::CharType);

        lookup.insert(config.keywords.r#if.clone(), Token::If);
        lookup.insert(config.keywords.r#else.clone(), Token::Else);
        lookup.insert(config.keywords.r#while.clone(), Token::While);
        lookup.insert(config.keywords.r#for.clone(), Token::For);
        lookup.insert(config.keywords.switch.clone(), Token::Switch);
        lookup.insert(config.keywords.case.clone(), Token::Case);
        lookup.insert(config.keywords.default.clone(), Token::Default);
        lookup.insert(config.keywords.function.clone(), Token::Function);
        lookup.insert(config.keywords.r#return.clone(), Token::Return);

        lookup.insert(config.literals.r#true.clone(), Token::BoolLiteral(true));
        lookup.insert(config.literals.r#false.clone(), Token::BoolLiteral(false));

        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            keywords_lookup: lookup
        }
    }

    fn next_token(&mut self) -> Result<TokenData, LexicalError> {
        self.skip_whitespace();
        let start_col = self.column;

        if self.is_at_end() {
            return Ok(self.emit(Token::EOF, start_col));
        }

        let c: char = self.peek();

        if c.is_alphabetic() {
            return self.read_identifier(start_col);
        }

        if c.is_ascii_digit() {
            return self.read_number(start_col);
        }

        match c {
            '(' => {
                self.advance();
                Ok(self.emit(Token::LeftParen, start_col))
            }
            ')' => {
                self.advance();
                Ok(self.emit(Token::RightParen, start_col))
            }
            '{' => {
                self.advance();
                Ok(self.emit(Token::LeftBrace, start_col))
            }
            '}' => {
                self.advance();
                Ok(self.emit(Token::RightBrace, start_col))
            }
            '[' => {
                self.advance();
                Ok(self.emit(Token::LeftBracket, start_col))
            }
            ']' => {
                self.advance();
                Ok(self.emit(Token::RightBracket, start_col))
            }
            ',' => {
                self.advance();
                Ok(self.emit(Token::Comma, start_col))
            }

            // Operadores lógicos (&& y ||)
            '&' => {
                self.advance();
                if self.match_next('&') {
                    self.advance();
                    Ok(self.emit(Token::And, start_col))
                } else {
                    let msg = format!("Expected '&' after '&'");
                    return Err(LexicalError::new(&msg, self.line, self.column));
                }
            }
            '|' => {
                self.advance();
                if self.match_next('|') {
                    self.advance();
                    Ok(self.emit(Token::Or, start_col))
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
                    return Ok(self.emit(Token::Equal, start_col));
                } else {
                    return Ok(self.emit(Token::Assign, start_col));
                }
            }
            '<' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(self.emit(Token::LessEqual, start_col));
                } else {
                    return Ok(self.emit(Token::Less, start_col));
                }
            }
            '>' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(self.emit(Token::GreaterEqual, start_col));
                } else {
                    return Ok(self.emit(Token::Greater, start_col));
                }
            }
            '!' => {
                self.advance();
                if self.match_next('=') {
                    self.advance();
                    return Ok(self.emit(Token::NotEqual, start_col));
                } else {
                    return Ok(self.emit(Token::Not, start_col));
                }
            }

            // Aritmetic Operators
            '+' => {
                self.advance();
                return Ok(self.emit(Token::Plus, start_col));
            }
            '-' => {
                self.advance();
                return Ok(self.emit(Token::Minus, start_col));
            }
            '*' => {
                self.advance();
                return Ok(self.emit(Token::Multiply, start_col));
            }
            '/' => {
                self.advance();
                if self.match_next('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    return self.next_token()
                } else {
                    Ok(self.emit(Token::Divide, start_col))
                }
            }
            '%' => {
                self.advance();
                return Ok(self.emit(Token::Modulo, start_col));
            }
            '^' => {
                self.advance();
                return Ok(self.emit(Token::Power, start_col));
            }

            // Other Operators
            ';' => {
                self.advance();
                return Ok(self.emit(Token::Semi, start_col));
            }
            '"' => return self.read_string(start_col),
            '\'' => return self.read_char(start_col),

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
            self.advance();
        }
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.input[self.position] == expected
    }

    fn read_identifier(&mut self, start_col: usize) -> Result<TokenData, LexicalError> {
        let start: usize = self.position;
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }
        let identifier: String = self.input[start..self.position].iter().collect();

        let kind = if let Some(token_kind) = self.keywords_lookup.get(&identifier) {
            token_kind.clone()
        } else {
            Token::Identifier(identifier)
        };
        Ok(self.emit(kind, start_col))
    }

    fn read_char(&mut self, start_col: usize) -> Result<TokenData, LexicalError> {
        self.advance();
        if self.is_at_end() {
            let msg = format!("Unterminated character literal");
            return Err(LexicalError::new(&msg, self.line, start_col));
        }
        let content: char = self.advance();

        if self.peek() != '\'' {
            let msg = format!("Invalid character literal");
            return Err(LexicalError::new(&msg, self.line, start_col));
        }
        self.advance();
        return Ok(self.emit(Token::CharLiteral(content), start_col));
    }

    fn read_string(&mut self, start_col: usize) -> Result<TokenData, LexicalError> {
        self.advance();
        let start: usize = self.position;
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        if self.is_at_end() {
            let msg = format!("Unterminated string literal");
            return Err(LexicalError::new(&msg, self.line, start_col));
        }
        let content: String = self.input[start..self.position].iter().collect();
        self.advance();
        return Ok(self.emit(Token::StringLiteral(content), start_col));
    }

    fn read_number(&mut self, start_col: usize) -> Result<TokenData, LexicalError> {
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
                return Err(LexicalError::new(&msg, self.line, start_col));
            }
            while !self.is_at_end() && self.input[self.position].is_ascii_digit() {
                self.advance();
            }
        }
        let literal: String = self.input[start..self.position].iter().collect();

        if !self.is_at_end() && self.input[self.position].is_alphabetic() {
            return Err(LexicalError::new(
                format!("Invalid number literal at position {}", self.position).as_str(),
                self.line,
                start_col,
            ));
        }

        if is_float {
            match literal.parse::<f64>() {
                Ok(n) => Ok(self.emit(Token::FloatLiteral(n), start_col)),
                Err(_) => Err(LexicalError::new(
                    "Invalid float literal",
                    self.line,
                    start_col,
                )),
            }
        } else {
            match literal.parse::<i64>() {
                Ok(n) => Ok(self.emit(Token::Integer(n), start_col)),
                Err(_) => Err(LexicalError::new(
                    "Invalid integer literal",
                    self.line,
                    start_col,
                )),
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

    pub fn tokenize(&mut self) -> Vec<TokenData> {
        let mut tokens: Vec<TokenData> = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.kind == Token::EOF;
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
